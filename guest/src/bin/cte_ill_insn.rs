//! CTE 测试：ILLEGAL_INSN 事件（非法指令）。
//!
//! 插入一条保留编码 0x0000（c.addi4spn nzuimm=0），触发 Event::IllInsn，
//! handler 推进 mepc 后继续执行。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, halt, trap_init};

static mut FLAG: bool = false;

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::IllInsn {
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
        core::arch::asm!(".half 0x0000", options(nostack),);
    }

    if unsafe { FLAG } { halt(0) } else { halt(1) }
}
