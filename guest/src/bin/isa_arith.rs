#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let a: u32 = 6;
    let b: u32 = 3;

    if a + b != 9 { rremu_am::halt(1); }
    if a - b != 3 { rremu_am::halt(1); }
    if a * b != 18 { rremu_am::halt(1); }
    if a / b != 2 { rremu_am::halt(1); }
    if a % b != 0 { rremu_am::halt(1); }

    // M 扩展：乘法与除法
    let c: u32 = 7;
    if c * c != 49 { rremu_am::halt(1); }
    if c * 0 != 0 { rremu_am::halt(1); }

    rremu_am::halt(0)
}
