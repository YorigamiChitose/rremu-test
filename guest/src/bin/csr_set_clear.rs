//! CSR 测试：CSRRS / CSRRC 位置位与清除。
//!
//! 读取 mie 初值 → csrs 置位 bit 3/7/11 → csrrs 读回 → csrc 清除 → 读回验证。

#![no_std]
#![no_main]
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { rremu_am::halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let before: u32; let after_set: u32; let after_clear: u32;
    let mask: u32 = 0x888; // bits 3,7,11 (MSIE/MTIE/MEIE)
    unsafe {
        core::arch::asm!(
            "csrr {before}, mie",
            "csrs mie, {mask}",
            "csrr {after_set}, mie",
            "csrc mie, {mask}",
            "csrr {after_clear}, mie",
            before = out(reg) before,
            after_set = out(reg) after_set,
            after_clear = out(reg) after_clear,
            mask = in(reg) mask,
        );
    }
    if before == 0 && after_set == 0x888 && after_clear == 0 {
        rremu_am::halt(0)
    } else {
        rremu_am::halt(1)
    }
}
