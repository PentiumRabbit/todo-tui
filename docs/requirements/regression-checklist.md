# 回归清单

> 只追加，不清空。历史回归记录永久保留。

---

## 2026-05-18 · FEAT-todo-tui-001

| 回归点 | 受影响模块 | 结果 | 来源 |
|--------|----------|------|------|
| `todo-tui add <title>` 正常写入并打印确认，exit 0 | core | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| `todo-tui add` 无 title 时 exit 1，不写入数据库 | core | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| `-p`/`-t`/`-d` 合法参数正确写入 | core | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| 非法 `-p`/`-d` 值打印错误，exit 1 | core | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| DB 不存在时自动初始化（CLI add 路径） | core | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| CLI add 后 TUI 在 1 秒内自动显示新条目 | core + app | ✅ 通过（逻辑审查） | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| TUI 处于 Add/Edit 模式时，外部写入不中断表单 | app | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| 表单关闭后，延迟的刷新立即生效 | app | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| 刷新后当前选中条目保持（条目仍存在时） | app | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| 数据库读取失败时 TUI 不崩溃 | app | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
| 现有 TUI 功能（CRUD、过滤、排序、搜索）不受影响 | core + app | ✅ 通过 | FEAT-todo-tui-001 / docs/architecture/reviews/FEAT-todo-tui-001-review.md |
