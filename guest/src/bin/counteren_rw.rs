//! Counter-enable 回归：mcounteren/scounteren 可读写并保持值。

#![no_std]
#![no_main]

use rremu_am::halt;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let m: u32;
    let s: u32;
    unsafe {
        core::arch::asm!(
            "li t0, 7",
            "csrw mcounteren, t0",
            "csrr {m}, mcounteren",
            "csrw scounteren, t0",
            "csrr {s}, scounteren",
            m = out(reg) m,
            s = out(reg) s,
            out("t0") _,
        );
    }
    if m == 7 && s == 7 { halt(0) } else { halt(1) }
}
