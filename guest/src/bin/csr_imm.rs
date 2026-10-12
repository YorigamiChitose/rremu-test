//! CSR 测试：CSR 立即数形式（CSRRSI / CSRRCI）。
//!
//! csrsi mie, 0x8 置位 bit3 → 读回 → csrcli mie, 0x8 清除 → 读回验证。

#![no_std]
#![no_main]
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { rremu_am::halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let before: u32; let after_set: u32; let after_clear: u32;
    unsafe {
        core::arch::asm!(
            "csrr {before}, mie",
            "csrsi mie, 0x8",
            "csrr {after_set}, mie",
            "csrci mie, 0x8",
            "csrr {after_clear}, mie",
            before = out(reg) before,
            after_set = out(reg) after_set,
            after_clear = out(reg) after_clear,
        );
    }
    if before == 0 && after_set == 0x8 && after_clear == 0 {
        rremu_am::halt(0)
    } else {
        rremu_am::halt(1)
    }
}
