# todo-tui 生命周期设计

**架构师**: 架构师角色
**日期**: 2026-04-23
**Step**: 4/6

---

## 应用生命周期

```
启动
 │
 ├─ 1. 确定数据库路径（~/.todo-tui/todos.db）
 ├─ 2. 创建目录（~/.todo-tui/）
 ├─ 3. Storage::new() → 打开/创建数据库，执行 schema 迁移
 ├─ 4. AppState::new(storage) → 加载所有数据到内存
 ├─ 5. 初始化 crossterm（raw mode + alternate screen）
 ├─ 6. 初始化 ratatui Terminal
 │
运行（事件循环）
 │
 ├─ loop {
 │     terminal.draw(|f| ui::render(f, &app))   // 渲染当前帧
 │     event = crossterm::event::read()          // 阻塞等待事件（超时 16ms）
 │     match app.handle_event(event) {
 │         AppAction::Continue => continue,
 │         AppAction::Quit => break,
 │     }
 │  }
 │
关闭
 │
 ├─ 1. 恢复终端（disable raw mode + leave alternate screen）
 ├─ 2. Storage 自动 drop（rusqlite Connection 关闭）
 └─ 3. 进程退出（exit code 0）
```

---

## 错误处理生命周期

```
任意阶段发生不可恢复错误
 │
 ├─ 1. 恢复终端（必须先恢复，否则终端损坏）
 ├─ 2. 打印错误信息到 stderr
 └─ 3. 进程退出（exit code 1）
```

**关键约束**：终端恢复必须在任何 panic 或错误退出前执行。使用 `std::panic::set_hook` 注册 panic 处理器，确保 `disable_raw_mode()` 和 `LeaveAlternateScreen` 被调用。

---

## 状态持久化时机

| 操作 | 持久化时机 |
|------|---------|
| 添加 todo | 用户按 Enter 确认后立即写入 |
| 编辑 todo | 用户按 Enter 确认后立即写入 |
| 删除 todo | 用户按 y/Enter 确认后立即写入 |
| 切换完成状态 | 按 Space 后立即写入 |
| 添加分类 | 确认后立即写入 |
