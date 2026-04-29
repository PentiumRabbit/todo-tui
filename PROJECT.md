# todo-tui 项目规则宪章

**技术栈**: Rust 2021 + ratatui 0.29 + crossterm 0.28 + rusqlite 0.32  
**目标平台**: macOS / Linux，终端最低 80×24

---

## 一、强制规则

### 代码质量

- **M-001** 禁止 `.unwrap()` / `.expect()` 出现在非测试代码中；可恢复错误必须返回 `Result`
- **M-002** 函数不超过 50 行；超过时拆分为独立函数
- **M-003** 公有函数必须有 doc 注释（`///`）
- **M-004** 禁止硬编码颜色 RGB 值；所有颜色常量集中到 `src/ui/theme.rs`
- **M-005** 提交前必须通过 `make check`（fmt + clippy + test）

### 架构约束

- **M-010** 单向数据流：事件 → AppState → Storage → 重渲染；UI 层只读 AppState，禁止写入
- **M-011** 所有数据库操作封装在 `src/storage/` 内，其他模块不得直接使用 rusqlite
- **M-012** 跨多个数据库表的写操作必须在事务内执行
- **M-013** `models/` 模块无副作用，不依赖 I/O 或数据库

### 测试

- **M-020** 新增功能必须包含单元测试
- **M-021** Storage 层每个公有方法必须有集成测试（使用临时数据库）
- **M-022** AppState 核心逻辑（过滤、状态转换）必须有单元测试

### 数据库

- **M-030** schema 变更通过版本化迁移（`schema_version` 表），禁止直接改已有迁移
- **M-031** 数据库文件路径：`~/.todo-tui/todos.db`，通过 `dirs-next` 获取 home 目录

---

## 二、目录结构

```
src/
├── main.rs          # 入口：终端初始化 + 事件循环
├── app.rs           # AppState：状态机 + 事件处理
├── models/
│   ├── mod.rs       # 公共类型导出
│   └── todo.rs      # Todo / Priority / TodoStatus 数据模型
├── storage/
│   └── mod.rs       # Storage：全部 SQLite CRUD
└── ui/
    ├── mod.rs        # 顶层渲染入口 + 布局
    ├── theme.rs      # 颜色常量（唯一来源）
    ├── list.rs       # Todo 列表面板
    ├── tags.rs       # 标签侧边栏
    ├── detail.rs     # 详情弹窗
    ├── form.rs       # 添加/编辑弹窗
    └── help.rs       # 快捷键帮助弹窗

tests/
├── integration_test.rs   # Storage 集成测试
└── app_state_tests.rs    # AppState 单元测试

docs/
├── architecture/    # 架构设计文档
├── interfaces/      # 模块接口契约
├── ui/              # UI 设计文档
└── ux/              # 交互设计文档

requirements/
├── requirements.md  # 需求文档
└── progress.md      # 进度跟踪
```

---

## 三、提交规范

遵循 Conventional Commits：

```
feat(scope): 添加新功能
fix(scope): 修复 bug
refactor(scope): 重构，不改行为
test(scope): 添加或修改测试
docs(scope): 文档更新
chore(scope): 构建/工具变更
```

scope 取值：`app` / `storage` / `ui` / `models` / `deps`
