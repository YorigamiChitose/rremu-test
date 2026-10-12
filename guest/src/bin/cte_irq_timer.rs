//! CTE 测试：IRQ_TIMER 事件（机器定时器中断）。
//!
//! 设置 mtimecmp = mtime + 10，启用 MIE + MTIE，等待中断触发。
//! handler 收到 Event::IrqTimer 后设置标志位，并将 mtimecmp 设为最大值
//! 以清除 pending（定时器中断为电平触发，不清除会导致 mret 后重入）。

#![no_std]
#![no_main]
use rremu_am::{Context, Event, trap_init, halt, clint_get_time, clint_set_timercmp};
static mut FLAG: bool = false;
extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::IrqTimer {
        unsafe {
            FLAG = true;
            // 清除 pending：mtimecmp = u64::MAX 使 mtime < mtimecmp
            clint_set_timercmp(u64::MAX);
        }
    }
    ctx
}
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(handler);
    let t = clint_get_time();
    clint_set_timercmp(t + 10);
    unsafe {
        core::arch::asm!(
            "csrsi mstatus, 0x8",
            "li t0, 0x80",
            "csrs mie, t0",
            out("t0") _,
            options(nostack),
        );
    }
    for _ in 0..10000u32 {
        if unsafe { FLAG } { halt(0) }
    }
    halt(1)
}
