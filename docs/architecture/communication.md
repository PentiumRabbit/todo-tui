# todo-tui 模块通信设计

**架构师**: 架构师角色
**日期**: 2026-04-23
**Step**: 5/6

---

## 通信模式

todo-tui 是单线程同步应用，所有模块通信均为**同步函数调用**，无消息队列或异步通道。

---

## 数据流向

```
crossterm 事件
       │
       ▼  KeyEvent
  app::handle_event()
       │
       ├─── 读操作 ──▶ app.todos / app.categories（内存缓存）
       │
       └─── 写操作 ──▶ storage.insert/update/delete()
                           │
                           ▼
                       SQLite 文件
                           │
                    写入成功后更新
                           │
                           ▼
                    app.todos 内存缓存
```

---

## 关键调用路径

### 添加 Todo
```
handle_event(Enter) 
  → validate_form() 
  → storage.insert_todo(&new_todo)
  → app.todos.push(inserted_todo)   // 更新内存缓存
  → app.mode = AppMode::Normal
```

### 删除 Todo
```
handle_event(y/Enter in DeleteConfirm)
  → storage.delete_todo(selected_id)
  → app.todos.remove(selected_index)  // 更新内存缓存
  → adjust_selected_index()
  → app.mode = AppMode::Normal
```

### 切换完成状态
```
handle_event(Space in Normal)
  → todo.completed = !todo.completed
  → storage.update_todo(&todo)
  → app.todos[selected_index] = todo  // 更新内存缓存
```

---

## 内存缓存策略

- 启动时一次性加载所有 todo 和 category 到内存
- 写操作：先写数据库，成功后再更新内存缓存（数据库为 source of truth）
- 无后台刷新（单用户应用，无并发修改场景）
