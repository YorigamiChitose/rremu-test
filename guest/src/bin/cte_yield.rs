//! CTE 测试：YIELD 事件（ecall, a7=u32::MAX）。
//!
//! 验证 ecall 触发陷阱后，handler 收到 Event::Yield，
//! 并在推进 mepc 后正常返回继续执行。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, trap_init, halt};

static mut FLAG: bool = false;

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::Yield {
        unsafe {
            FLAG = true;
            // 推进 mepc 跳过 ecall 指令
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

    let magic: u32 = u32::MAX; // YIELD
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") magic,
            options(nostack),
        );
    }

    if unsafe { FLAG } {
        halt(0)
    } else {
        halt(1)
    }
}
