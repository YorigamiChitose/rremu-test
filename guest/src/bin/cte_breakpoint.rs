//! CTE 测试：BREAKPOINT 事件（ebreak）。
//!
//! 验证 ebreak 触发 Event::Breakpoint，handler 推进 mepc 后继续执行。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, halt, trap_init};

static mut FLAG: bool = false;

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::Breakpoint {
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
        core::arch::asm!("ebreak", options(nostack),);
    }

    if unsafe { FLAG } { halt(0) } else { halt(1) }
}
