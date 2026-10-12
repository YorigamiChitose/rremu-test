# rremu-test

rremu 模拟器的**端到端固件级测试工程**。Cargo workspace，包含两个成员：

| 成员 | 说明 |
|---|---|
| `guest/` | `no_std` RV32 测试二进制 crate（`riscv32imac-unknown-none-elf`），每个 `src/bin/*.rs` 是一个独立测试用例，链接 [rremu-am](../rremu-am) HAL |
| `harness/` | host 侧运行器：解析 `guest/Tests.toml` → 构建 guest → 加载 ELF → 经 rremu-machine 执行并断言结果 |

## 运行

```sh
cd harness && cargo run --release --bin rremu-harness
```

依赖：stable Rust 1.99+、`riscv32imac-unknown-none-elf` target、rremu 主仓库（path 依赖）。

## 测试规格（Tests.toml）

每条 `[[test]]`：

| 字段 | 说明 |
|---|---|
| `name` | 测试名（= guest bin 名 = ELF 文件名） |
| `expect` | `"Pass"`（exit 0）/ `"Fail"`（exit 非 0）/ `"Fault:<substr>"` / `"Timeout"` |
| `max_instrs` | 指令数上限，超出判 Paused |
| `uart_preload` | 可选，运行前预注入 UART RX 的字节串 |
| `uart_expect` | 可选，运行结束后 UART 输出缓冲须与此串精确匹配 |
| `fb` | 可选，为 true 时映射 framebuffer 设备 |

## 用例清单（26 个）

| 分类 | 用例 |
|---|---|
| ISA（4） | isa_arith / isa_logical / isa_load_store / isa_branch |
| 设备（3） | uart_output / uart_input / clint_timer |
| CTE（12） | cte_yield / cte_syscall / cte_breakpoint / cte_ill_insn / cte_load_fault / cte_store_fault / cte_misalign_load / cte_misalign_store / cte_irq_timer / cte_irq_software / cte_ctx_switch / cte_preserve_regs |
| CSR（5） | csr_rw / csr_set_clear / csr_imm / csr_counters / csr_mhartid |
| 边界（2） | edge_div_zero / edge_null_addr |

## 关键约定

- 退出：guest 调 `rremu_am::halt(code)`（ecall + a7=EXIT_MAGIC），harness 断言退出码
- 中断类用例的 handler 须清除电平触发源（`clint_set_timercmp(u64::MAX)` / `clint_set_msip(0)`）再返回，否则 mret 后重入
- 异常类用例的 handler 须按指令长度推进 `mepc`（取指首 half 低 2 位判 2/4 字节）跳过故障指令
- `edge_null_addr` 无 trap handler，预期 Timeout（mtvec=0 自旋）
