# 测试用例文档

**项目**: todo-tui
**设计**: 测试设计师
**日期**: 2026-04-29

---

## REQ-008：左右键面板切换

| 用例ID | 名称 | 关联需求 | 前置条件 | 测试步骤 | 预期结果 | 严重程度 | 状态 |
|--------|------|---------|---------|---------|---------|---------|------|
| TC-001 | Normal 模式下 `←` 切换到标签面板 | REQ-008 矩阵行1 | 应用启动，Normal 模式，焦点在 Todo 面板（focus_tag_panel=false） | 按 `←` | focus_tag_panel=true，标签面板边框高亮 | P1 | 已通过 |
| TC-002 | 标签面板下 `→` 切换回 Todo 面板 | REQ-008 矩阵行2 | focus_tag_panel=true | 按 `→` | focus_tag_panel=false，Todo 面板边框高亮 | P1 | 已通过 |
| TC-003 | Todo 面板下按 `→` 无响应 | REQ-008 矩阵行3 | Normal 模式，focus_tag_panel=false | 按 `→` | focus_tag_panel 保持 false，无任何变化 | P2 | 已通过 |
| TC-004 | 标签面板下按 `←` 无响应 | REQ-008 矩阵行4 | focus_tag_panel=true | 按 `←` | focus_tag_panel 保持 true，无任何变化 | P2 | 已通过 |
| TC-005 | Search 模式下 `←` 不触发面板切换 | REQ-008 矩阵行5 | AppMode::Search | 按 `←` | 不切换面板，Search 模式继续，search_query 不变 | P1 | 已通过 |
| TC-006 | Detail 模式下方向键不触发面板切换 | REQ-008 矩阵行6 | AppMode::Detail | 分别按 `←` 和 `→` | Detail 模式不受影响，焦点状态不变 | P1 | 已通过 |
| TC-007 | Form 模式（Add/Edit）下方向键不触发面板切换 | REQ-008 矩阵行6 | AppMode::Add 或 AppMode::Edit | 按 `←` / `→` | 表单不关闭，焦点不切换 | P1 | 已通过 |
| TC-008 | DeleteConfirm 模式下方向键不触发面板切换 | REQ-008 矩阵行6 | AppMode::DeleteConfirm | 按 `←` / `→` | 确认框不关闭，焦点不切换 | P1 | 已通过 |
| TC-009 | 已在标签面板连续按 `→` 幂等 | REQ-008 矩阵行7 | focus_tag_panel=false（已在 Todo 面板） | 连续按 3 次 `→` | 每次结果均为 focus_tag_panel=false，无副作用 | P2 | 已通过 |
| TC-010 | `←` 与 `Tab` 切换到标签面板效果一致 | REQ-008 矩阵行8 | Normal 模式，focus_tag_panel=false | 先按 `Tab` 确认切换，Esc 回来；再按 `←` 确认切换 | 两次结果均为 focus_tag_panel=true | P2 | 已通过 |
| TC-011 | `→` 与 `Tab` 切换回 Todo 面板效果一致 | REQ-008 矩阵行8 | focus_tag_panel=true | 先按 `Tab` 确认切换回，再重进标签面板按 `→` 确认 | 两次结果均为 focus_tag_panel=false | P2 | 已通过 |
| TC-012 | 状态栏提示正确显示 `[←/Tab]` | REQ-008 验收标准 | Normal 模式，focus_tag_panel=false | 观察底部状态栏 | 显示 `[←/Tab] 标签栏` | P2 | 已通过 |
| TC-013 | 状态栏提示正确显示 `[→/Tab]` | REQ-008 验收标准 | focus_tag_panel=true | 观察底部状态栏 | 显示 `[→/Tab] 切回列表` | P2 | 已通过 |
