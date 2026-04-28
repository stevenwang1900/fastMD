use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use tauri::Emitter;

#[derive(Serialize, Deserialize, Debug)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

// 启动时要打开的文件路径（来自命令行参数）
struct OpenFile(Mutex<Option<String>>);
// 用于缓存启动时收到的文件打开事件（RunEvent::Opened 在 setup 之前触发）
struct PendingOpenFile(Arc<Mutex<Option<String>>>);

#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_text_file(path: String, content: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_dir(path: String) -> Result<Vec<DirEntry>, String> {
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let meta = entry.metadata().map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        let full_path = entry.path().to_string_lossy().to_string();
        result.push(DirEntry {
            name,
            path: full_path,
            is_dir: meta.is_dir(),
        });
    }
    result.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            if a.is_dir { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    Ok(result)
}

#[tauri::command]
fn create_dir_cmd(path: String) -> Result<(), String> {
    fs::create_dir_all(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_path(path: String, is_dir: bool) -> Result<(), String> {
    if is_dir {
        fs::remove_dir_all(&path).map_err(|e| e.to_string())
    } else {
        fs::remove_file(&path).map_err(|e| e.to_string())
    }
}

#[tauri::command]
fn rename_path(old_path: String, new_path: String) -> Result<(), String> {
    fs::rename(&old_path, &new_path).map_err(|e| e.to_string())
}

/// 前端启动后调用，获取并消费启动时要打开的文件路径
/// 优先返回命令行参数，其次返回 RunEvent::Opened 收到的路径
#[tauri::command]
fn get_open_file(
    cmd_line: tauri::State<OpenFile>,
    pending: tauri::State<PendingOpenFile>,
) -> Option<String> {
    // 先检查命令行参数
    if let Some(path) = cmd_line.0.lock().unwrap().take() {
        return Some(path);
    }
    // 再检查 RunEvent::Opened 缓存的路径
    pending.0.lock().unwrap().take()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 从命令行参数里提取 .md/.markdown 文件路径
    let args: Vec<String> = std::env::args().collect();
    eprintln!("[MD Editor] 启动参数: {:?}", args);
    
    let open_path: Option<String> = args
        .iter()
        .skip(1)
        .find(|a| {
            let low = a.to_lowercase();
            low.ends_with(".md") || low.ends_with(".markdown")
        })
        .cloned();
    
    eprintln!("[MD Editor] 解析到的文件路径: {:?}", open_path);

    let open_path = Mutex::new(open_path);
    // 用于缓存启动时收到的文件打开事件（前端可能还没准备好）
    let pending_open_file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let pending_open_file_for_run = pending_open_file.clone();

    tauri::Builder::default()
        .manage(OpenFile(open_path))
        .manage(PendingOpenFile(pending_open_file))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            read_text_file,
            write_text_file,
            read_dir,
            create_dir_cmd,
            remove_path,
            rename_path,
            get_open_file,
        ])

        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |_app_handle, event| {
            // 处理 macOS 文件打开事件
            #[cfg(target_os = "macos")]
            match &event {
                tauri::RunEvent::Opened { urls } => {
                    eprintln!("[MD Editor] RunEvent::Opened 触发, urls: {:?}", urls);
                    for url in urls {
                        eprintln!("[MD Editor] 处理 URL: {:?}", url);
                        if let Ok(path) = url.to_file_path() {
                            let path_str = path.to_string_lossy().to_string();
                            eprintln!("[MD Editor] 解析到路径: {}", path_str);
                            // 尝试发送事件给前端
                            let result = _app_handle.emit("open-file", path_str.clone());
                            eprintln!("[MD Editor] emit 结果: {:?}", result);
                            // 无论成功与否，都缓存一份供 get_open_file 使用
                            //（如果 emit 成功，前端会收到事件；如果失败，前端可以通过 get_open_file 获取）
                            *pending_open_file_for_run.lock().unwrap() = Some(path_str);
                        } else {
                            eprintln!("[MD Editor] URL 转路径失败: {:?}", url);
                        }
                    }
                }
                _ => {}
            }
        });
}
