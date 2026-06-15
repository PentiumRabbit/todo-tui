# CLAUDE.md — AI 工作指南

## 项目概述

todo-tui 是 Rust TUI 待办事项应用。技术栈：ratatui + crossterm + rusqlite。

## 强制规则

读取 `PROJECT.md` 了解所有强制约束。核心要点：

1. **禁止 unwrap**：非测试代码中一律返回 `Result`
2. **UI 只读**：`src/ui/` 内禁止写入 `AppState` 任何字段
3. **颜色集中**：颜色常量只在 `src/ui/theme.rs` 定义，其他文件引用
4. **提交前**：必须通过 `make check`

## 质量检查命令

```bash
make check        # fmt + clippy + test（提交前必须全部通过）
make test         # 仅跑测试
make lint         # 仅跑 clippy
cargo run         # 手动验证 UI
```

## 架构要点

- 单向数据流：`crossterm 事件 → AppState → Storage → ratatui 渲染`
- 所有 DB 操作封装在 `src/storage/mod.rs`
- 颜色主题：`src/ui/theme.rs`
- 状态机：`AppMode` 枚举，转换逻辑集中在 `src/app.rs`

## 测试

- Storage 集成测试：`tests/integration_test.rs`（用 tempfile 隔离）
- AppState 单元测试：`tests/app_state_tests.rs`
- 模型测试：各 `src/models/*.rs` 文件内 `#[cfg(test)]`
