#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let a: u32 = 0xF0;
    let b: u32 = 0x0F;

    if a & b != 0x00 { rremu_am::halt(1); }
    if a | b != 0xFF { rremu_am::halt(1); }
    if a ^ b != 0xFF { rremu_am::halt(1); }
    if !a != 0xFFFFFF0F { rremu_am::halt(1); }

    let s: u32 = 0x01;
    if s << 4 != 0x10 { rremu_am::halt(1); }
    if 0x80 >> 4 != 0x08 { rremu_am::halt(1); }

    let neg: i32 = -8;
    if (neg >> 1) != -4 { rremu_am::halt(1); }

    rremu_am::halt(0)
}
