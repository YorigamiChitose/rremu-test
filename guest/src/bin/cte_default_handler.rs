//! CTE 测试：默认陷阱处理（未安装 handler 时）。
//!
//! 不调 trap_init，直接执行非法指令触发 ILLEGAL_INSTR，应被 AM 启动时
//! 默认安装的 mtvec 捕获，默认 handler 记录 mcause/mepc 后
//! halt(DEFAULT_TRAP_EXIT=0x10)，表现为非零退出（Fail）。

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // 不调 trap_init，触发非法指令 → 默认 handler → halt(0x10)
    unsafe {
        core::arch::asm!(
            ".half 0x0000", // c.addi4spn nzuimm=0 → 非法
            options(nostack),
        );
    }
    // 不应到达（默认 handler 已 halt）
    rremu_am::halt(0)
}
