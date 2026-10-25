//! FP e2e：FS=Off 时访问 fcsr 触发 IllInsn。
#![no_std]
#![no_main]

use core::ptr::read_volatile;
use rremu_am::{Context, Event, halt, trap_init};

static mut CAUGHT: bool = false;

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::IllInsn {
        unsafe {
            CAUGHT = true;
            let mepc = (*ctx).mepc;
            let half = read_volatile(mepc as *const u16);
            let len = if half & 0x3 == 0x3 { 4 } else { 2 };
            (*ctx).mepc = mepc.wrapping_add(len);
        }
    }
    ctx
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(handler);
    unsafe {
        core::arch::asm!("csrr t0, fcsr", out("t0") _, options(nostack));
    }
    if unsafe { CAUGHT } && rremu_am::fp::read_fs() == rremu_am::fp::FS_OFF {
        halt(0)
    } else {
        halt(1)
    }
}
