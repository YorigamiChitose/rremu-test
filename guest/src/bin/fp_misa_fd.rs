//! FP e2e：misa 必须同时 advertise F 与 D，且 D 出现时 F 存在。
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let misa: u32;
    unsafe {
        core::arch::asm!("csrr {misa}, misa", misa = out(reg) misa, options(nostack));
    }
    let f = 1u32 << 5;
    let d = 1u32 << 3;
    if misa & f != 0 && misa & d != 0 && (misa >> 30) & 0b11 == 0b01 {
        rremu_am::halt(0)
    } else {
        rremu_am::halt(1)
    }
}
