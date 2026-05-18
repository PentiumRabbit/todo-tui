# DISPATCH#28 — dev-lead 执行计划

**角色**: dev-lead-FEAT-todo-tui-001-28
**任务**: 统筹实现 FEAT-todo-tui-001（CLI 快捷添加 + TUI 自动刷新）
**需求文档**: docs/requirements/FEAT-todo-tui-001.md
**制定日期**: 2026-05-18

---

## 一、任务分析

### 需求概要

FEAT-todo-tui-001 包含两项独立功能：

**FR-1：CLI 快捷添加**
- 新增 `todo-tui add <title>` 子命令
- 支持 `-p`（优先级）、`-t`（标签，可多次）、`-d`（截止时间）短 flag
- 完整的错误处理与边界校验
- 影响模块：`core`（main.rs 入口/CLI 解析）、`storage`（复用 insert_todo）、`models`（复用 NewTodo/Priority）

**FR-2：TUI 自动刷新**
- 每 500ms 轮询 DB 文件 mtime
- 检测到变化后重新加载 todo 列表
- 刷新不中断 Add/Edit 表单；保持当前选中条目
- 影响模块：`core`（main.rs 事件循环）、`app`（AppState 新增刷新逻辑）

### 模块影响评估

| 模块 | FR-1 | FR-2 | 备注 |
|------|------|------|------|
| core (main.rs) | 新增 CLI 路由 | 新增 mtime 轮询 | 两处改动均在 main.rs，需同一工程师处理 |
| app.rs | 无 | 新增 reload + 防中断逻辑 | 独立改动 |
| storage | 复用 insert_todo（无需改动） | 复用 list_todos（无需改动） | 不需要单独委派 |
| models | 复用 NewTodo/Priority（无需改动） | 无 | 不需要单独委派 |

### 复杂度判断

- 涉及 2 个模块有实质改动（core/main.rs、app.rs）
- 无跨模块接口变更
- 需要架构评审确认：CLI 路由方案（是否引入 clap）、mtime 轮询集成方式、刷新防中断机制

---

## 二、执行路径规划

```
📋 研发负责人 执行路径规划

接收任务: FEAT-todo-tui-001（CLI 快捷添加 + TUI 自动刷新）
来源: 总负责人（DISPATCH#28）
输入: docs/requirements/FEAT-todo-tui-001.md

端属性判断:
  变更范围: 单一 Rust 二进制，无前后端分层
  委派对象: 单一模块架构师（涉及 core + app，两个模块但单一代码库，无跨层冲突）

委派清单:
  Step 1 · 架构评审（先行）
    ├─ 委派对象: arch-FEAT-todo-tui-001（模块架构师）
    ├─ 任务: 评审 CLI 子命令路由方案、mtime 轮询集成方式、刷新防中断机制
    ├─ 交付物: docs/architecture/reviews/FEAT-todo-tui-001-review.md
    └─ 下级可继续委派: 否

  Step 2 · 实现（依赖 Step 1 N2 通过）
    ├─ 委派对象: eng-FEAT-todo-tui-001-cli（工程师，负责 FR-1）
    │   任务: 实现 CLI add 子命令（main.rs 路由 + 参数解析 + 错误处理）
    │   交付物: src/main.rs 改动 + 集成测试
    ├─ 委派对象: eng-FEAT-todo-tui-001-tui（工程师，负责 FR-2）
    │   任务: 实现 TUI 自动刷新（app.rs 刷新逻辑 + main.rs 轮询集成）
    │   交付物: src/app.rs + src/main.rs 改动 + 单元测试
    └─ 并行无干扰依据（待 arch review.md 确认后填写）

  Step 3 · 测试验证（依赖 Step 2）
    ├─ 委派对象: 测试负责人
    ├─ 任务: 回归验证 FR-1 和 FR-2 验收标准
    └─ 交付物: 测试报告

执行顺序: Step 1 → N2 → Step 2 → Step 3 → N3
```

**注意**：Step 2 的 eng-cli 和 eng-tui 是否可并行，取决于 arch review.md 中对 main.rs 改动范围的界定：
- 若 FR-1 和 FR-2 在 main.rs 的改动区域不交叉 → 可并行
- 若有交叉 → 串行（eng-cli 先，eng-tui 后）
- 此判断由架构师在 review.md 中明确给出

---

## 三、执行步骤

1. [x] 读 all.md + dev-lead.md
2. [x] 读需求文档 FEAT-todo-tui-001.md
3. [x] 检查 MODULE-TREE.md（不存在 → 已制定并写入）
4. [x] 写本计划文件
5. [ ] 写入 dispatch 行：arch-FEAT-todo-tui-001（架构评审）→ 写入 DISPATCH#N
6. [ ] 退出，等总负责人启动架构师 Agent
7. [ ] 收到架构师 delivered 后：执行 CHECKLISTS.md §一点五 核查
8. [ ] N2 自动通过后：写入工程师 dispatch 行
9. [ ] 收到工程师 delivered 后：委派测试负责人
10. [ ] 收到测试负责人汇报通过后：N3，汇报总负责人
11. [ ] 更新交付清单，set-status delivered，发送 📬

---

## 四、当前状态

已完成步骤 1–4，下一步：写入架构师 dispatch 行后退出。
