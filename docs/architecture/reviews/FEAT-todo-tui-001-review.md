# FEAT-todo-tui-001 架构评审

> 架构师: arch-FEAT-todo-tui-001-29
> REQ: FEAT-todo-tui-001
> 日期: 2026-05-18
> 状态: 已确认

---

## 一、需求摘要

在单一 Rust 二进制中新增 `todo-tui add` CLI 子命令（FR-1）和 TUI 自动刷新（FR-2）。
前者要求在 `main()` 入口引入 CLI 路由分支；后者要求在现有 16ms 事件循环中叠加 500ms mtime 检测，并在表单模式下延迟刷新。两项功能均集中在 `core`（main.rs）和 `app`（app.rs），不涉及 storage/models 的接口变更。

---

## 二、模块影响分析

| 模块/文件 | 变更类型 | 说明 |
|-----------|---------|------|
| `src/main.rs` | 修改 | 新增 CLI 路由逻辑（FR-1）；新增 mtime 轮询计时器（FR-2） |
| `src/app.rs` | 修改 | 新增 `reload_from_db()` 方法；新增 `pending_reload` 标志及表单关闭时触发逻辑 |
| `src/storage/mod.rs` | 无变更 | `insert_todo()` 和 `list_todos()` 直接复用 |
| `src/models/mod.rs` | 无变更 | `NewTodo`、`Priority` 直接复用 |
| `Cargo.toml` | 修改（可选） | 若采用 clap 方案则新增依赖；若手动解析则无变更 |

**模块边界判断**：
- FR-1 的 CLI 路由和 FR-2 的 mtime 轮询均在 `main.rs` 内，但改动区域不交叉：
  - FR-1 改动位于 `main()` 函数开头（参数检测 → 早退出），在 TUI 初始化之前
  - FR-2 改动位于 `run()` 函数内部（事件循环计时器）
  - **结论：FR-1 和 FR-2 在 main.rs 的改动区域不交叉，两个工程师可并行实现**
- `app.rs` 的 reload 逻辑仅由 FR-2 涉及，FR-1 无需触碰

---

## 三、功能分层设计

| 功能点 | 落层 | 理由 |
|--------|------|------|
| CLI 参数解析 | core（main.rs） | 二进制入口职责，在 TUI 初始化前完成 |
| CLI 写入数据库 | 数据层（storage） | 复用已有 `insert_todo()`，无需新增接口 |
| mtime 轮询计时 | core（main.rs） | 事件循环驻留在 main.rs，计时逻辑自然归属此处 |
| 重新加载 todo 列表 | 业务逻辑层（app.rs） | AppState 持有 storage 引用，reload 是状态更新操作 |
| 刷新防中断标志 | 业务逻辑层（app.rs） | AppMode 判断和 pending_reload 均属状态机逻辑 |
| TUI 渲染 | UI 层（ui/） | 无变更，渲染层透明感知数据变化 |

---

## 四、状态管理设计

**新增/修改的状态**：

| 状态名 | 类型 | 归属 | 共享范围 | 说明 |
|--------|------|------|---------|------|
| `pending_reload` | `bool` | `AppState`（app.rs） | 仅 AppState 内部 | 表单模式下触发刷新时置 true，表单关闭时执行实际 reload |
| `last_mtime` | `Option<SystemTime>` | `run()` 局部变量（main.rs） | 仅 run() 函数内 | 记录上次检测到的 DB 文件 mtime，用于变化检测 |
| `mtime_tick` | `u32`（计数器） | `run()` 局部变量（main.rs） | 仅 run() 函数内 | 累积 16ms tick，达到 ~31 次（约 500ms）时触发检测 |

**状态通信方式**：
- main.rs 检测到 mtime 变化后，直接调用 `app.trigger_reload()` 方法（同步调用，无 channel）
- `trigger_reload()` 根据当前 `app.mode` 决定立即执行 reload 或设置 `pending_reload = true`

---

## 五、数据流设计

**FR-1 CLI add 数据流**：

```
用户执行 todo-tui add <title> [flags]
  │
  ▼
main() 检测到 args[1] == "add"
  │
  ▼
解析 title / -p / -t / -d 参数
  │ 参数非法 → 打印错误，exit(1)
  ▼
Storage::new(db_path)  // 自动初始化（与 TUI 路径一致）
  │
  ▼
Storage::insert_todo(&NewTodo { title, priority, tags, due_date, notes })
  │ 写入失败 → 打印错误，exit(非0)
  ▼
打印确认信息（含 ID 或 title），exit(0)
// 进程退出，不进入 TUI
```

**FR-2 TUI 自动刷新数据流**：

```
run() 事件循环（每 16ms）
  │
  ├─ event::poll(16ms) → 处理键盘/鼠标事件（现有逻辑不变）
  │
  └─ mtime_tick += 1
       │ tick < 31 → 继续下一轮
       ▼ tick >= 31（约 500ms）
     mtime_tick = 0
     读取 DB 文件 mtime
       │ mtime 未变 → 继续
       ▼ mtime 变化
     app.trigger_reload()
       │
       ├─ mode == Add | Edit → pending_reload = true（延迟）
       │
       └─ 其他 mode → 立即执行 reload_from_db()
                         │
                         ▼
                       storage.list_todos() + list_all_tags()
                         │
                         ▼
                       更新 app.todos + app.all_tags
                       保持 selected_index（若条目仍存在）
```

**表单关闭时的延迟刷新触发**：

```
handle_form(Esc / Enter 提交) → mode = Normal
  │
  └─ submit_form() 或 mode 切回 Normal 后
       检查 pending_reload == true
         │
         ▼
       执行 reload_from_db()，清除 pending_reload
```

---

## 六、接口契约

**AppState 新增方法**（app.rs）：

```rust
// 触发刷新入口：由 main.rs 调用
// 若处于 Add/Edit 模式则设置 pending_reload，否则立即执行
pub fn trigger_reload(&mut self) -> Result<()>

// 实际执行重新加载：从 storage 读取最新 todos 和 tags
// 保持 selected_index 对应条目不变（按 todo.id 查找）
fn reload_from_db(&mut self) -> Result<()>
```

**AppState 新增字段**（app.rs）：

```rust
pub struct AppState {
    // ... 现有字段 ...
    pub pending_reload: bool,  // 表单模式下的延迟刷新标志
}
```

**main.rs 新增 CLI 路由**：

```rust
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "add" {
        return run_cli_add(&args[2..]);
    }
    // ... 现有 TUI 启动逻辑 ...
}

fn run_cli_add(args: &[String]) -> Result<()>
```

**main.rs run() 函数新增变量**：

```rust
fn run(...) -> Result<()> {
    // ... 现有局部变量 ...
    let mut mtime_tick: u32 = 0;
    let mut last_mtime: Option<std::time::SystemTime> = None;
    // ...
}
```

---

## 七、可复用组件 / 公共逻辑识别

| 候选项 | 当前位置 | 复用场景 | 提取建议 |
|--------|---------|---------|---------|
| `db_path()` 函数 | main.rs | CLI add 和 TUI 启动均需要 DB 路径 | 留原位，两处均可调用（已在 main.rs 顶层） |
| `Storage::insert_todo()` | storage/mod.rs | CLI add 直接复用，无需修改 | 留原位，不提取 |
| `models::NewTodo` + `Priority` | models/todo.rs | CLI add 直接复用，无需修改 | 留原位，不提取 |
| `FormState::parse_due_date()` | models/mod.rs | CLI add 的 `-d` 参数解析可复用此逻辑 | 留原位，CLI add 直接调用 |

**提取决策**：
- 不提取：所有候选项均已在合理位置，CLI add 作为 main.rs 内的独立函数可直接调用现有接口，无需新增共享层。
- `db_path()` 已在 main.rs 顶层，FR-1 和 FR-2 均可访问，无需移动。

---

## 八、方案对比

### 方案对比一：CLI 路由方案

| 维度 | 方案 A：手动解析 args | 方案 B：引入 clap |
|------|---------------------|-----------------|
| 描述 | 直接检测 `args[1] == "add"`，手动解析 `-p`/`-t`/`-d` flag | 引入 clap crate，声明式定义子命令和 flag |
| 优点 | 零依赖，编译时间短，代码量约 60–80 行，完全可控 | 自动生成 `--help`，flag 解析健壮，扩展性好 |
| 缺点 | 需手动处理 flag 顺序/重复（`-t` 多次），错误信息需自行格式化 | 新增约 1.5MB 编译产物，引入外部依赖，当前需求规模不匹配 |
| 适用条件 | 子命令只有 `add`，flag 数量固定（3个），不在范围内扩展其他子命令 | 子命令多（≥3个）或 flag 复杂时 |
| 推荐 | ✅ | ❌ |

**推荐方案**：方案 A（手动解析 args）

**推荐理由**：需求明确限定"不在范围"只有 `add` 子命令，flag 数量固定（`-p`/`-t`/`-d`），手动解析完全够用。引入 clap 是过度设计，增加编译时间和二进制体积，与项目"轻量终端工具"定位不符。`-t` 多次使用通过遍历 args 即可处理，不构成手动解析的难点。

---

### 方案对比二：mtime 轮询集成方式

| 维度 | 方案 A：事件循环内计数器 | 方案 B：独立线程 + channel |
|------|----------------------|--------------------------|
| 描述 | 在 `run()` 循环内维护 `mtime_tick` 计数器，每 16ms +1，累积到 ~31 次（约 500ms）时执行 mtime 检测 | 启动独立线程，每 500ms 检测一次 mtime，通过 `std::sync::mpsc::channel` 向主线程发送刷新信号 |
| 优点 | 单线程，无同步原语，代码简单（约 15 行），与现有事件循环模型一致 | 计时精确，与事件循环解耦，易于未来扩展为 inotify/FSEvents |
| 缺点 | 计时不精确（依赖 poll 实际耗时），误差约 ±16ms，在 500ms 量级完全可接受 | 需要 `Arc<Mutex<>>` 或 channel，增加代码复杂度；crossterm 的 terminal 不可跨线程共享，需额外设计 |
| 适用条件 | 轮询间隔 ≥ 100ms（误差比例小），单线程模型，不需要 inotify 精度 | 需要精确计时或未来替换为 OS 文件监听（inotify/kqueue） |
| 推荐 | ✅ | ❌ |

**推荐方案**：方案 A（事件循环内计数器）

**推荐理由**：500ms 轮询间隔对 ±16ms 误差不敏感（误差比 3.2%），单线程模型与 ratatui/crossterm 的设计一致（两者均非线程安全），避免引入 Mutex/channel 的复杂性。需求验收标准是"1 秒内刷新"，方案 A 完全满足。未来若需要精确文件监听，可独立立项替换，不影响当前实现。

---

### 方案对比三：刷新防中断机制

| 维度 | 方案 A：pending_reload 标志 | 方案 B：每轮均 reload，表单模式跳过 UI 更新 |
|------|---------------------------|------------------------------------------|
| 描述 | AppState 新增 `pending_reload: bool`；表单模式下触发刷新时只置标志；表单关闭（Esc/提交）时检查并执行实际 reload | 每次检测到 mtime 变化都调用 `reload_from_db()`，但在 Add/Edit 模式下跳过 `app.todos` 的更新，仅记录"有变化"的状态 |
| 优点 | 语义清晰：reload 只发生在安全时机；表单内数据完全隔离，无竞态 | 数据始终最新，表单关闭后立即可见 |
| 缺点 | 若用户长时间处于表单模式，reload 延迟累积（但需求只要求"不中断"，不要求实时） | 实现较复杂：需区分"跳过 todos 更新"和"跳过渲染"两个层面；且 reload_from_db 仍然执行 I/O（浪费） |
| 适用条件 | 需求要求"不中断表单"，允许表单期间延迟刷新 | 需要表单关闭后零延迟看到最新数据（需求未要求） |
| 推荐 | ✅ | ❌ |

**推荐方案**：方案 A（pending_reload 标志）

**推荐理由**：需求 FR-2 明确要求"不中断当前操作（如正在输入的表单）"，方案 A 语义完全匹配。方案 B 在表单模式下仍执行 I/O 是不必要的开销，且逻辑更复杂。`pending_reload` 标志实现简单（约 10 行），在 `submit_form()` 和表单 Esc 处理中各加一次检查即可。

---

## 九、风险评估

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|---------|
| CLI add 与 TUI 同时写入同一 DB（并发写） | 低 | 中 | SQLite WAL 模式默认支持并发读写；rusqlite 连接各自独立，无共享状态；需求场景是"CLI 写完后 TUI 读"，非真正并发 |
| mtime 检测在 macOS 上精度不足（HFS+ 1s 精度） | 中 | 低 | macOS 默认文件系统（APFS）mtime 精度为纳秒；HFS+ 已淘汰。若遇到精度问题，可在 CLI add 后额外 touch DB 文件（不在当前范围） |
| `pending_reload` 标志在长表单操作后积压多次变化 | 低 | 低 | 标志只是 bool，多次触发只执行一次 reload（最终一致），符合需求 |
| FR-1 和 FR-2 在 main.rs 合并冲突（并行实现时） | 中 | 低 | 改动区域已明确不交叉（FR-1 在 main() 开头，FR-2 在 run() 内部），工程师按此边界实现可避免冲突 |
| `-d` 日期解析：`FormState::parse_due_date()` 接受 `YYYY-MM-DD HH:MM`，CLI 需求也是此格式 | 低 | 低 | 直接复用，无风险；需在 CLI 错误信息中明确告知用户格式 |

---

## 十、实现任务拆分

| # | 任务描述 | 负责角色 | 涉及文件 | 依赖 | 可并行 |
|---|---------|---------|---------|------|--------|
| T1 | 实现 CLI add 子命令：main() 路由 + run_cli_add() 函数 + 参数解析 + 错误处理 + 集成测试 | 工程师（cli） | `src/main.rs`、`Cargo.toml`（无需修改） | 无 | ✅ |
| T2 | 实现 TUI 自动刷新：AppState::trigger_reload() + reload_from_db() + pending_reload 标志 + main.rs 计时器集成 + 单元测试 | 工程师（tui） | `src/app.rs`、`src/main.rs`（run() 函数） | 无 | ✅ |
| T3 | 回归验证：FR-1 和 FR-2 验收标准全覆盖 | 测试执行者 | — | T1 + T2 | ❌ |

**并行说明**：T1 修改 `main()` 函数开头（早退出逻辑），T2 修改 `run()` 函数内部和 `app.rs`，两处在 `main.rs` 中不交叉，可并行实现。合并时按文件区域 merge 即可，无冲突风险。

---

## 模块列表

本次涉及以下模块（后续所有角色按此命名产出摘要文件）：

| 模块名称 | 模块描述 | 摘要文件举例 |
|---------|---------|------------|
| core | 应用入口、CLI 路由、TUI 事件循环、mtime 轮询（src/main.rs） | arch-core.md / eng-core.md |
| app | 应用状态机、reload 逻辑、防中断标志（src/app.rs） | arch-app.md / eng-app.md |

**不涉及模块**：storage、models、ui 本次均无接口变更，不需要产出摘要。

---

## 回归影响分析

本次变更影响以下回归点（测试执行者回归时必须覆盖）：

| 回归点 | 受影响模块 | 回归优先级 |
|--------|----------|-----------|
| `todo-tui add <title>` 正常写入并打印确认，exit 0 | core | P0 |
| `todo-tui add` 无 title 时 exit 1，不写入数据库 | core | P0 |
| `-p`/`-t`/`-d` 合法参数正确写入 | core | P0 |
| 非法 `-p`/`-d` 值打印错误，exit 1 | core | P1 |
| DB 不存在时自动初始化（CLI add 路径） | core | P1 |
| CLI add 后 TUI 在 1 秒内自动显示新条目 | core + app | P0 |
| TUI 处于 Add/Edit 模式时，外部写入不中断表单 | app | P0 |
| 表单关闭后，延迟的刷新立即生效 | app | P1 |
| 刷新后当前选中条目保持（条目仍存在时） | app | P1 |
| 数据库读取失败时 TUI 不崩溃 | app | P1 |
| 现有 TUI 功能（CRUD、过滤、排序、搜索）不受影响（回归） | core + app | P1 |
