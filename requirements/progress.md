# todo-tui 进度跟踪

**最后更新**: 2026-05-15
**整体进度**: 100%（M1~M5 全部完成，REQ-013/014 已合入）

---

## 里程碑

| 里程碑 | 完成日期 | 状态 |
|--------|---------|------|
| M1: 项目脚手架 + 基础 CRUD | 2026-04-23 | 完成 ✓ |
| M2: 分类/优先级/截止日期 | 2026-04-23 | 完成 ✓ |
| M3: SQLite 持久化 | 2026-04-23 | 完成 ✓ |
| M4: 完整 TUI 界面 | 2026-04-23 | 完成 ✓ |
| M5: 测试与发布 | 2026-04-23 | 完成 ✓ |

---

## 已完成功能

- [x] Cargo.toml + 依赖初始化（ratatui / crossterm / rusqlite / anyhow / chrono）
- [x] 数据模型：`Todo`、`Priority`、`TodoStatus`（含 `is_overdue`、`is_due_today`）
- [x] SQLite 存储层：`Storage` CRUD + 版本化迁移（schema_version）
- [x] AppState 状态机：`AppMode` 枚举，键盘事件处理，标签/搜索过滤
- [x] TUI 渲染层：列表面板、标签侧边栏、详情弹窗、表单弹窗、帮助弹窗
- [x] 集成测试：`tests/integration_test.rs`（tempfile 隔离）

---

## 当前已知技术债

| 问题 | 严重程度 | 计划修复 |
|------|---------|---------|
| `storage/mod.rs` 第 165-166 行有 `.unwrap()` | 高 | Round 2 |
| `ui/mod.rs` 写入 `AppState.layout_*` 字段（违反单向数据流） | 高 | Round 2 |
| 删除操作未包裹在事务中 | 中 | Round 2 |
| `render_list()` / `render_detail_popup()` 超 50 行 | 中 | Round 3 |
| 缺少 `AppState` 单元测试 | 中 | Round 2 |
| 颜色常量散落在各 UI 文件（未集中到 theme.rs） | 中 | Round 3 |

---

## 风险与问题

暂无阻塞项。

---

## 下阶段计划（工程治理改造）

- **Round 2**：消除高风险代码缺陷（unwrap、MVC 违规、缺测试）
- **Round 3**：代码质量提升（函数拆分、theme.rs 颜色集中、doc 注释）
