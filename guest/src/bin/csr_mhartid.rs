//! CSR 测试：mhartid 只读且复位值为 0。

#![no_std]
#![no_main]
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { rremu_am::halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let id: u32;
    unsafe {
        core::arch::asm!(
            "csrr {id}, mhartid",
            id = out(reg) id,
        );
    }
    if id == 0 { rremu_am::halt(0) } else { rremu_am::halt(1) }
}
