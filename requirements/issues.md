# 问题追踪

**项目**: todo-tui
**维护**: 测试执行者

---

| Issue ID | 关联用例 | 严重程度 | 标题 | 复现步骤 | 实际结果 | 预期结果 | 状态 | 指派给 |
|---------|---------|---------|------|---------|---------|---------|------|------|
| ISS-001 | — | P1 | CI 首次运行失败：clippy useless_vec + checkout Node.js 警告 | 推送代码触发 CI | clippy 报 `useless use of vec!`，exit code 101；checkout 报 Node.js 20 deprecation 警告 | CI 全绿通过 | 已关闭 | 研发负责人 |
| ISS-002 | — | P1 | tag v0.4.1 触发 CI 失败：README.md 未提及版本号 | git tag v0.4.1 → push → CI 触发 release workflow | `Error: README.md does not mention version 0.4.1` exit code 1，阻塞发布 | CI 通过，release 正常发布 | 已修复 | dev-lead-ISS002-65 |
| ISS-003 | — | P1 | CI 失败：Clippy int_plus_one + items_after_test_module + Format 不合规 | push 触发 CI | Clippy: app.rs:864 int_plus_one / main.rs:133 items_after_test_module；Format: app.rs:132 单行 if 需拆多行 | CI 全绿 | 待修复 | — |
