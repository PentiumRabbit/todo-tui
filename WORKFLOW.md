# 工程执行流程

---

## 标准变更流程

```
需求明确
  │
  ├─ 影响架构边界（新模块/接口变更）→ 先更新 docs/architecture/
  │
  ├─ 功能变更 → 先写测试，再实现，最后更新 requirements/progress.md
  │
  └─ Bug 修复 → 先写回归测试复现，再修复
```

---

## 编码工作流

### 1. 开始前

- 确认变更在 `requirements/requirements.md` 中有对应需求条目
- 如影响 Storage 接口，先更新 `docs/interfaces/storage.interface.md`
- 如影响 AppState 公有 API，先更新 `docs/interfaces/app.interface.md`

### 2. 实现

- 遵循 `PROJECT.md` 所有强制规则（M-001 ~ M-031）
- 颜色只从 `src/ui/theme.rs` 引用，禁止在 UI 文件中硬编码 RGB
- Storage 操作失败必须通过 `AppState.error_message` 展示给用户

### 3. 提交前

```bash
make check   # cargo fmt --check + cargo clippy + cargo test
```

全部通过后提交，禁止 `--no-verify` 跳过。

---

## 测试策略

| 层次 | 位置 | 工具 |
|------|------|------|
| 模型单元测试 | `src/models/todo.rs` 内 `#[cfg(test)]` | cargo test |
| AppState 单元测试 | `tests/app_state_tests.rs` | cargo test |
| Storage 集成测试 | `tests/integration_test.rs` | cargo test + tempfile |

UI 渲染不做自动化测试，通过手动运行 `cargo run` 验证。

---

## 数据库迁移规则

1. 新建迁移：在 `storage/mod.rs` 的 `migrate()` 末尾追加 `if version < N` 块
2. 禁止修改已有迁移块
3. 迁移必须幂等（`CREATE TABLE IF NOT EXISTS`、`INSERT OR IGNORE`）

---

## 版本历史

| 版本 | 日期 | 变更 |
|------|------|------|
| 1.0 | 2026-04-29 | 初始版本 |
