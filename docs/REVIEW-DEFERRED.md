# rremu-test 第三层审查暂缓说明

> 状态：**暂缓**。
>
> 本文件只记录暂缓范围，不修改 `guest/`、`harness/`、`Tests.toml` 或测试代码。

## 1. 当前状态

`rremu-test` 当前有：

```text
54 个端到端用例
```

覆盖：

- M-mode ISA / CTE / CSR
- S-mode sret / CSR / sfence / 委派
- S 中断 software / timer / external
- PMP load / store
- Sv32 identity / page fault / store page fault
- RTC / syscon / poweroff / reboot
- 未实现 CSR illegal
- 启动注入 / DTB / load base

## 2. 暂缓原因

第一层核心仍在进行：

- 模块边界未最终确定
- CPU / Bus / Machine 会继续拆分
- rremu-am 的全局状态会收敛
- 公共接口可能需要显式化

此时扩展测试容易绑定到临时的内部结构。

## 3. 暂缓范围

暂缓：

- 新增 guest 用例
- harness 结构重构
- `Tests.toml` 扩展
- 测试矩阵扩展
- 性能/覆盖率优化

不暂缓：

- 现有测试作为第一层重构的回归资产
- 第一层重构后重新运行全部 54 个用例

## 4. 恢复条件

满足以下条件后恢复第三层：

```text
rremu/rremu-am 文档完成
CPU / Bus / Machine / Config 拆分完成
公共接口和 ABI 稳定
第一层单元测试通过
```

## 5. 恢复后的优先级

1. 重新运行现有 54 个用例
2. 为拆分后的接口补契约测试
3. 为 rremu-am 状态收敛补回归
4. 再评估新增端到端场景

## 6. 约束

- 不删除现有用例
- 不降低现有断言强度
- 不把第二层 `platforms/` 场景混入第三层
- 不把临时调试测试直接变成永久测试
