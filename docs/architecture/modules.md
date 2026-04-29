# todo-tui 模块拆分设计

**架构师**: 架构师角色
**日期**: 2026-04-23
**Step**: 2/6

---

## 模块总览

```
src/
├── main.rs          # 入口：初始化 + 事件循环
├── app.rs           # AppState 模块：状态机核心
├── models/
│   ├── mod.rs
│   ├── todo.rs      # Todo 数据模型
│   └── category.rs  # Category 数据模型
├── storage/
│   ├── mod.rs
│   ├── db.rs        # Database 连接管理
│   ├── todo_repo.rs # Todo CRUD 仓储
│   └── category_repo.rs # Category CRUD 仓储
└── ui/
    ├── mod.rs
    ├── render.rs    # 顶层渲染入口
    ├── list.rs      # 列表面板组件
    ├── detail.rs    # 详情面板组件
    ├── form.rs      # 添加/编辑弹窗
    └── help.rs      # 帮助弹窗
```

---

## 模块详情

### 模块 1：`app`（应用状态机）

**职责**：持有全局应用状态，处理所有键盘事件，协调 storage 和 ui。

**能力清单**：
- 持有 `AppMode` 枚举（当前应用模式）
- 持有 todo 列表缓存（`Vec<Todo>`）
- 持有当前选中索引
- 接收键盘事件，分发到对应处理逻辑
- 调用 storage 层执行持久化

**依赖**：`models`、`storage`
**被依赖**：`main`、`ui`

**接口契约**：见 `docs/interfaces/app.interface.md`

---

### 模块 2：`models`（数据模型）

**职责**：定义纯数据结构，无副作用，无 I/O。

**能力清单**：
- `Todo` 结构体：id, title, completed, priority, category_id, due_date, created_at
- `Category` 结构体：id, name, color
- `Priority` 枚举：High, Medium, Low
- `AppMode` 枚举：Normal, Add, Edit, DeleteConfirm, Search, Help

**依赖**：无（仅标准库 + chrono + serde）
**被依赖**：`app`、`storage`、`ui`

---

### 模块 3：`storage`（数据持久化）

**职责**：封装所有 SQLite 操作，提供 CRUD 接口。

**能力清单**：
- 数据库初始化（建表、迁移）
- Todo CRUD：`insert`, `update`, `delete`, `list_all`, `find_by_id`
- Category CRUD：`insert`, `update`, `delete`, `list_all`
- 按分类过滤查询

**依赖**：`models`、rusqlite
**被依赖**：`app`

**接口契约**：见 `docs/interfaces/storage.interface.md`

---

### 模块 4：`ui`（渲染层）

**职责**：将 `AppState` 渲染为 ratatui 组件树，只读访问状态，不修改状态。

**能力清单**：
- 主界面渲染（两栏布局）
- 列表面板渲染（含高亮、颜色、滚动）
- 详情面板渲染
- 添加/编辑弹窗渲染
- 删除确认弹窗渲染
- 帮助弹窗渲染
- 响应式布局（根据终端尺寸调整）

**依赖**：`models`、ratatui
**被依赖**：`main`

**接口契约**：见 `docs/interfaces/ui.interface.md`

---

## 模块依赖图

```
main ──▶ app ──▶ storage ──▶ models
         │                    ▲
         └──▶ ui ─────────────┘
```

（单向依赖，无循环）
