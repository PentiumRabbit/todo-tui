# todo-tui 项目团队定义

| 字段       | 内容                                           |
| ---------- | ---------------------------------------------- |
| 项目名称   | todo-tui                                       |
| 项目路径   | /Users/pr/Work/GitSource/todo-tui              |
| 项目负责人 | proj-lead                                      |
| 创建日期   | 2026-05-28                                     |
| 最后更新   | 2026-05-28                                     |

---

## 角色分配表

| 通用角色名     | role_key      | 本项目 assignee 前缀 | 负责模块/范围  | 备注                        |
| -------------- | ------------- | -------------------- | -------------- | --------------------------- |
| 项目负责人     | proj-lead     | proj-lead            | 整个项目       |                             |
| PM 产品经理    | pm            | pm                   | 需求管理       |                             |
| 研发负责人     | dev-lead      | dev-lead             | 研发统筹       | 直接委派模块架构师和工程师   |
| 测试负责人     | test-lead     | test-lead            | 测试统筹       |                             |
| core 架构师    | arch-core     | arch-core            | main.rs / app.rs / config.rs / i18n.rs | Rust TUI |
| models 架构师  | arch-models   | arch-models          | src/models/    | 纯数据结构，无 I/O          |
| storage 架构师 | arch-storage  | arch-storage         | src/storage/   | SQLite CRUD / 迁移          |
| ui 架构师      | arch-ui       | arch-ui              | src/ui/        | ratatui 渲染层              |
| core 工程师    | eng-core      | eng-core             | main.rs / app.rs / config.rs / i18n.rs | Rust TUI |
| models 工程师  | eng-models    | eng-models           | src/models/    |                             |
| storage 工程师 | eng-storage   | eng-storage          | src/storage/   |                             |
| ui 工程师      | eng-ui        | eng-ui               | src/ui/        |                             |

---

## 模块结构

> 首次架构评审确定，见 `docs/engineering/MODULE-TREE.md`。

```text
todo-tui
├─ core    (src/main.rs, src/app.rs, src/config.rs, src/i18n.rs)
├─ models  (src/models/)
├─ storage (src/storage/)
└─ ui      (src/ui/)
```

**本项目无模块组长**：规模小（4 个顶层模块），研发负责人直接委派模块架构师和工程师。

---

## 常设角色记录

| 角色名 | role_key | 首次启用日期 | 晋升日期 | 功能描述 |
| ------ | -------- | ------------ | -------- | -------- |
| —      | —        | —            | —        | 暂无常设临时角色 |
