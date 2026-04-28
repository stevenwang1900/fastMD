# 贡献指南

感谢你对 MD Editor 项目的关注！我们欢迎各种形式的贡献。

## 如何贡献

### 报告问题

如果你发现了 bug 或有功能建议，请通过 GitHub Issues 提交：

1. 检查是否已有类似 issue
2. 使用对应的 issue 模板
3. 提供详细的复现步骤和环境信息

### 提交代码

1. Fork 本仓库
2. 创建功能分支：`git checkout -b feature/你的功能名`
3. 提交更改：`git commit -m '描述你的更改'`
4. 推送到分支：`git push origin feature/你的功能名`
5. 创建 Pull Request

## 开发环境设置

```bash
# 克隆项目
git clone https://github.com/yourusername/md-editor.git
cd md-editor

# 开发模式（需要安装 Tauri CLI: cargo install tauri-cli）
cargo tauri dev

# 构建发布版本
cargo tauri build
```

## 代码规范

- 遵循 Rust 官方代码风格（使用 `cargo fmt` 和 `cargo clippy`）
- 添加适当的注释和文档
- 保持提交信息清晰明了

## 提交信息规范

使用以下格式：

```
<类型>: <描述>

[可选的详细描述]

[可选的关联 issue]
```

类型包括：
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具相关

## 行为准则

- 保持友善和尊重
- 接受建设性的批评
- 关注对社区最有利的事情

## 许可证

通过提交代码，你同意你的贡献将采用与项目相同的 [MIT 许可证](LICENSE)。
