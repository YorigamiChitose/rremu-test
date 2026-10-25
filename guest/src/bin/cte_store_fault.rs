//! CTE 测试：STORE_FAULT 事件（向未映射地址 store）。
#![no_std]
#![no_main]
use rremu_am::{Context, Event, halt, trap_init};
static mut FLAG: bool = false;
extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::StoreFault {
        unsafe {
            FLAG = true;
            let mepc = (*ctx).mepc;
            let half = core::ptr::read_volatile(mepc as *const u16);
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
        core::arch::asm!(
            "li t0, 0xDEAD0000",
            "sw zero, 0(t0)",
            out("t0") _,
            options(nostack),
        );
    }
    if unsafe { FLAG } { halt(0) } else { halt(1) }
}
