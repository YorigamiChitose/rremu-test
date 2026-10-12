//! CSR 测试：CSRRW 读写一致性。
//!
//! 向 mscratch 写入已知值后读回，验证 CSRRW 的读-写语义。

#![no_std]
#![no_main]
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { rremu_am::halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let w: u32 = 0xDEAD_BEEF;
    let r: u32;
    unsafe {
        core::arch::asm!(
            "csrw mscratch, {w}",
            "csrr {r}, mscratch",
            w = in(reg) w,
            r = out(reg) r,
        );
    }
    if r == 0xDEAD_BEEF { rremu_am::halt(0) } else { rremu_am::halt(1) }
}
