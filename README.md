# rremu-test

> ⚠️ **AI 产物声明**：本仓库的代码与文档主要由 AI 在人工需求约束、审查与测试下生成，
> 仅供学习/实验用途；不保证工程质量与长期维护。请以源码和测试结果为准。

rremu 模拟器的**端到端固件级测试工程**。Cargo workspace，包含两个成员：

| 成员 | 说明 |
|---|---|
| `guest/` | `no_std` RV32 测试二进制 crate（`riscv32imac-unknown-none-elf`），每个 `src/bin/*.rs` 是一个独立测试用例，链接 [rremu-am](../rremu-am) HAL |
| `harness/` | host 侧运行器：解析 `guest/Tests.toml` → 构建 guest → 加载 ELF → 经 rremu-machine（用 `rremu-config` 构造机器）执行并断言结果 |

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
| `expect` | `"Pass"`（exit 0）/ `"Fail"`（exit 非 0）/ `"Exit:<code>"` / `"Fault:<substr>"` / `"Timeout"` |
| `max_instrs` | 指令数上限，超出判 Paused |
| `uart_preload` | 可选，运行前预注入 UART RX 的字节串 |
| `uart_expect` | 可选，运行结束后 UART 输出缓冲须与此串精确匹配 |
| `fb` | 可选，为 true 时映射 framebuffer 设备 |
| `boot_hartid` / `boot_dtb` | 可选，覆盖启动时注入的 a0/a1 |
| `load_base` | 可选，将 ELF 段与入口整体平移到该基址 |
| `dtb_hex` | 可选，将十六进制 DTB 字节加载到 RAM 顶部并注入 boot a1 |

## 用例清单（57 个）

| 分类 | 用例 |
|---|---|
| 退出基线（1） | exit_normal |
| ISA（6） | isa_arith / isa_logical / isa_load_store / isa_branch / isa_amo / isa_lr_sc |
| 设备（3） | uart_output / uart_input / clint_timer |
| CTE（14） | cte_yield / cte_syscall / cte_breakpoint / cte_ill_insn / cte_load_fault / cte_store_fault / cte_misalign_load / cte_misalign_store / cte_irq_timer / cte_irq_software / cte_ctx_switch / cte_preserve_regs / cte_mtval / cte_default_handler |
| CSR（9） | csr_rw / csr_set_clear / csr_imm / csr_counters / csr_mhartid / csr_mip / csr_unimplemented / csr_sip_readonly / counteren_rw |
| S-mode（6） | smode_sret / smode_sret_u / smode_csr / smode_illegal_mcsr / smode_sfence / smode_delegate_ecall |
| S 中断（3） | s_irq_software / s_irq_timer / s_irq_external |
| PMP（3） | pmp_enforce_load / pmp_enforce_store / pmp_lock_write |
| RTC/syscon（2） | rtc_syscon / syscon_reboot |
| Sv32（3） | sv32_identity / sv32_page_fault / sv32_store_page_fault |
| 启动注入（3） | boot_args / load_base_pie / dtb_load |
| 断言矩阵（2） | fail_case（Fail）/ cte_vectored_mtvec（Fault） |
| 边界（2） | edge_div_zero / edge_null_addr |

## S2/S3 回归覆盖

| 缺陷类别 | 永久用例 |
|---|---|
| S-mode ecall | smode_delegate_ecall |
| CSR 权限 | smode_illegal_mcsr / smode_csr |
| 异常委派 | smode_delegate_ecall |
| PMP load/store | pmp_enforce_load / pmp_enforce_store |
| MMU page fault | sv32_page_fault / sv32_store_page_fault |
| S 中断 | s_irq_software / s_irq_timer / s_irq_external |
| CSR 存在性（S3.6） | csr_unimplemented |

## 关键约定

- 退出：guest 调 `rremu_am::halt(code)`（ecall + a7=EXIT_MAGIC），harness 断言退出码
- 中断类用例的 handler 须清除电平触发源（`clint_set_timercmp(u64::MAX)` / `clint_set_msip(0)`）再返回，否则 mret 后重入
- 异常类用例的 handler 须按指令长度推进 `mepc`（取指首 half 低 2 位判 2/4 字节）跳过故障指令
- `cte_mtval` 验证 trap 后 `mtval` 记录故障地址；`csr_mip` 验证 `mip` 的 MSIP 位由设备 pending 投影
- `csr_unimplemented` 验证未实现 CSR（如 `stimecmp`）读取触发 illegal，防止 OpenSBI 误判可选扩展
- `cte_default_handler` 验证未调 `trap_init` 时的默认陷阱处理：AM 启动装 mtvec，默认 handler 记录 mcause/mepc 后 `halt(0x10)`
- `edge_null_addr` 显式 `csrw mtvec, zero` 模拟无 handler，预期 Timeout（自旋）
- S-mode 用例经 `rremu_am::enter_supervisor` 从 M 进入 S；`guest/src/lib.rs` 的
  `supervisor_exit_handler` 把 S-mode `halt(code)` 的 ecall 桥接回 M-mode 后退出
- 委派类 S 陷阱经 `rremu_am::trap_init_s` / `SContext` 处理；`dtb_load` 验证
  `dtb_hex` 实际写入 RAM 顶部且 boot `a1` 指向该内容
- 失败用例会自动写出 `harness/dumps/<name>.json`，可用 `rremu-report`（rremu-am workspace）
  解析为可读报告（退出码语义、trap 原因、寄存器/trace）

## 跨仓库一致性

harness 同时依赖 `rremu-config`（path）与 `rremu-am`（path），其单测
`config_defaults_match_am_arch` 断言 `rremu-am/arch.rs`（人工镜像）与
`rremu-config` 默认值逐项相等。地址若在某侧漂移，此测试会立即失败。

```sh
cd harness && cargo test
```
