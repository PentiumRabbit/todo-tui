# DISPATCH#32 回归验证计划

> 角色: tester-FEAT-todo-tui-001-32
> 任务: FEAT-todo-tui-001 回归验证
> 日期: 2026-05-18

---

## 分析

本次回归验证覆盖 FEAT-todo-tui-001（CLI 快捷添加 + TUI 自动刷新）的所有验收标准，
依据架构评审文档"九、回归影响分析"逐条验证，并对照需求文档 FR-1 / FR-2 验收标准核查。

验证方式：
- `cargo build --release`：确认编译通过
- `cargo test`：确认所有单元/集成测试通过
- 手动调用 release binary 验证 FR-1 关键路径
- 阅读 src/app.rs 和 src/main.rs 确认 FR-2 逻辑正确性（无法启动交互式 TUI）

---

## 执行步骤

1. 读需求文档 FR-1 / FR-2 验收标准 ✅
2. 读架构评审回归影响分析 ✅
3. 执行 `cargo build --release`
4. 执行 `cargo test` 并分析测试覆盖率
5. 手动验证 FR-1 关键路径（共 6 个场景）：
   - 正常 add（写入 + 打印 + exit 0）
   - 空 title（exit 1，不写入）
   - 未提供 title（exit 1，不写入）
   - 非法 -p 值（exit 1）
   - -t 多次使用（多标签写入）
   - -d 格式错误（exit 1）
6. 阅读 src/app.rs 确认 FR-2 逻辑：
   - trigger_reload 在 Add/Edit 模式下置 pending_reload
   - 其他模式立即 reload_from_db
   - pending_reload 在 submit_form/Esc 后清除
7. 阅读 src/main.rs 确认 mtime 轮询计时器实现
8. 产出回归报告：docs/tasks/dispatch/32/regression-report.md
9. 更新 docs/requirements/regression-checklist.md

---

## 验收标准对照

| 验收标准 | 验证方式 |
|---------|---------|
| cargo build --release 通过 | 构建命令 |
| cargo test 全部通过 | 测试命令 |
| CLI add 关键路径验证 | 手动调用 binary |
| FR-2 单元测试覆盖核心逻辑 | cargo test 输出 + 代码阅读 |
| 现有 TUI 功能回归 | cargo test 通过即视为回归通过 |
| 产出回归报告 | 写入 docs/tasks/dispatch/32/regression-report.md |
