# rremu-test 第三层审查暂缓说明

> 状态：**部分恢复**：规范回归用例已补充，结构性扩展仍暂缓。
>
> 本文件记录第三层当前范围和恢复边界。

## 1. 当前状态

`rremu-test` 当前有：

```text
57 个端到端用例
```

覆盖：

- M-mode ISA / CTE / CSR
- S-mode sret / CSR / sfence / 委派
- S 中断 software / timer / external
- PMP load / store
- Sv32 identity / page fault / store page fault
- RTC / syscon / poweroff / reboot
- 未实现 CSR illegal
- sip 只读位 / PMP 锁定写保护 / counter-enable
- 启动注入 / DTB / load base

## 2. 当前阶段

第一层核心重构已完成，以下基础已经稳定：

- CPU / Bus / Machine / Config 已拆分
- rremu-am 全局状态已收敛到 `state.rs`
- 公共接口已显式化
- 规范回归已有 CPU / machine 单测

因此已恢复小范围 guest 规范回归；但暂不进行大规模 harness 重构或矩阵扩展。

## 3. 暂缓范围

暂缓：

- 大规模新增 guest 用例
- harness 结构重构
- 测试矩阵扩展
- 性能/覆盖率优化

不暂缓：

- 现有测试作为第一层重构的回归资产
- 运行全部 57 个用例

## 4. 恢复条件

基础条件已经满足：

```text
rremu/rremu-am 文档完成
CPU / Bus / Machine / Config 拆分完成
公共接口和 ABI 稳定
第一层单元测试通过
```

## 5. 当前优先级

1. 保持现有 57 个用例通过
2. 仅在发现真实规范缺口时补 guest 用例
3. 暂不扩大测试矩阵
4. 暂不重构 harness

## 6. 约束

- 不删除现有用例
- 不降低现有断言强度
- 不把第二层 `platforms/` 场景混入第三层
- 不把临时调试测试直接变成永久测试
