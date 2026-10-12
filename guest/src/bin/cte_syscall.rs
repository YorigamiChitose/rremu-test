//! CTE 测试：SYSCALL 事件（ecall, a7=42）。
//!
//! 验证 ecall 以非 YIELD/EXIT_MAGIC 的 a7 触发 Event::Syscall，
//! handler 在 Context 中设置 a0=0（成功），guest 取回返回值。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, trap_init, halt};

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::Syscall {
        unsafe {
            (*ctx).gpr[10] = 0; // a0 = 0 (成功)
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

    let syscall_num: u32 = 42;
    let result: u32;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") syscall_num,
            in("a0") 1u32,
            lateout("a0") result,
            options(nostack),
        );
    }

    if result == 0 {
        halt(0)
    } else {
        halt(1)
    }
}
