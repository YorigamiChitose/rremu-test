//! 断言矩阵测试：Fault（vectored mtvec 未实现）。
//!
//! 设置 mtvec 为 vectored 模式（bit0=1）后 ecall，触发 trap → rremu 返回
//! Faulted("vectored mtvec")。harness 断言 Expect::Fault:vectored mtvec。

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        core::arch::asm!(
            "li t0, 0x101", // mtvec = 0x101，bit0=1 → vectored
            "csrw mtvec, t0",
            "ecall",
            out("t0") _,
            options(nostack),
        );
    }
    // 不应到达（trap 已使 rremu 进入 Faulted）
    rremu_am::halt(1)
}
