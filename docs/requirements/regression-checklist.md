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

---

## 2026-05-28 · FEAT-todo-tui-002

| 回归点 | 受影响模块 | 结果 | 来源 |
|--------|----------|------|------|
| 进入 Search 模式时默认为普通模式，状态栏无 `[RE]` 指示符 | app + ui | ✅ 通过 | FEAT-todo-tui-002 / FR-1 |
| 按 `Ctrl+R` 后状态栏出现 `[RE]` 指示符，列表立即按正则重新过滤 | app + ui | ✅ 通过 | FEAT-todo-tui-002 / FR-1 |
| 再次按 `Ctrl+R`，指示符消失，列表恢复普通模式过滤结果 | app + ui | ✅ 通过 | FEAT-todo-tui-002 / FR-1 |
| 退出 Search 模式（ESC / Enter）后重新进入，正则模式重置为关闭 | app | ✅ 通过 | FEAT-todo-tui-002 / FR-1 |
| 正则 `^买` 仅匹配 title 以"买"开头的条目（边界匹配可用） | app | ✅ 通过 | FEAT-todo-tui-002 / FR-2 |
| 正则 `work\|urgent` 匹配 tags 含"work"或"urgent"的条目 | app | ✅ 通过 | FEAT-todo-tui-002 / FR-2 |
| 正则匹配覆盖 title、tags、notes 三个字段 | app | ✅ 通过 | FEAT-todo-tui-002 / FR-2 |
| 正则匹配大小写不敏感（`TODO` 匹配"todo"） | app | ✅ 通过 | FEAT-todo-tui-002 / FR-2 |
| 正则模式下输入非法正则（如 `[未闭合`），UI 出现可读错误提示，列表为空 | app + ui | ✅ 通过 | FEAT-todo-tui-002 / FR-3 |
| 非法正则补全后（如补 `]`），错误提示立即消失，列表恢复过滤 | app + ui | ✅ 通过 | FEAT-todo-tui-002 / FR-3 |
| 普通模式下输入正则特殊字符不出现错误提示 | app + ui | ✅ 通过 | FEAT-todo-tui-002 / FR-3 |
| 错误提示不导致 UI 布局错乱或程序崩溃 | ui | ✅ 通过 | FEAT-todo-tui-002 / FR-3 |
| 普通搜索（子串匹配）行为不受影响 | app | ✅ 通过 | FEAT-todo-tui-002 回归 |
