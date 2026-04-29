# 问题追踪

**项目**: todo-tui
**维护**: 测试执行者

---

| Issue ID | 关联用例 | 严重程度 | 标题 | 复现步骤 | 实际结果 | 预期结果 | 状态 | 指派给 |
|---------|---------|---------|------|---------|---------|---------|------|------|
| ISS-001 | — | P1 | CI 首次运行失败：clippy useless_vec + checkout Node.js 警告 | 推送代码触发 CI | clippy 报 `useless use of vec!`，exit code 101；checkout 报 Node.js 20 deprecation 警告 | CI 全绿通过 | 已关闭 | 研发负责人 |
