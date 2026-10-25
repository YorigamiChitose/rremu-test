//! CTE 测试：IRQ_SOFTWARE 事件（机器软件中断）。
//!
//! 启用 MIE + MSIE，设置 msip=1 触发软件中断。
//! handler 收到 Event::IrqSoftware 后设置标志位，并清除 msip
//! （软件中断为电平触发，不清除会导致 mret 后重入）。

#![no_std]
#![no_main]
use rremu_am::{Context, Event, clint_set_msip, halt, trap_init};
static mut FLAG: bool = false;
extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::IrqSoftware {
        unsafe {
            FLAG = true;
            clint_set_msip(0);
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
            "csrsi mstatus, 0x8",
            "li t0, 0x8",
            "csrs mie, t0",
            out("t0") _,
            options(nostack),
        );
    }
    clint_set_msip(1);
    for _ in 0..10000u32 {
        if unsafe { FLAG } {
            halt(0)
        }
    }
    halt(1)
}
