# ui 模块接口契约

**架构师**: 架构师角色
**日期**: 2026-04-23
**Step**: 3/6

---

## 对外方法

### `ui::render(frame: &mut Frame, app: &AppState)`
- 输入：ratatui Frame（当前帧），AppState 只读引用
- 输出：无（直接绘制到 frame）
- 约束：
  - 只读访问 AppState，不修改任何状态
  - 必须在 terminal.draw() 回调内调用
  - 根据 `app.mode` 决定渲染哪些弹窗层

---

## 内部组件函数（不对外暴露）

```rust
fn render_main_layout(frame: &mut Frame, app: &AppState, area: Rect)
fn render_todo_list(frame: &mut Frame, app: &AppState, area: Rect)
fn render_detail_panel(frame: &mut Frame, app: &AppState, area: Rect)
fn render_status_bar(frame: &mut Frame, app: &AppState, area: Rect)
fn render_form_popup(frame: &mut Frame, app: &AppState, area: Rect)
fn render_delete_confirm(frame: &mut Frame, app: &AppState, area: Rect)
fn render_help_popup(frame: &mut Frame, area: Rect)
```

---

## 渲染约束

- 所有弹窗使用 `ratatui::widgets::Clear` 清除背景后再绘制
- 列表滚动通过 `ListState` 管理，偏移量来自 `app.list_offset`
- 最小终端尺寸检查：宽 < 80 或高 < 24 时渲染警告覆盖层
