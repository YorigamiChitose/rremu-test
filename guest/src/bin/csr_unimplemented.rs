//! CSR 回归测试：未实现 CSR 的读取必须触发 illegal instruction。
//!
//! 该用例覆盖 S3.6 根因：读取 `stimecmp` 若返回 0，OpenSBI 会误判
//! SSTC 可用并把定时器写入该 CSR，导致 CLINT timer 永远不被编程。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, halt, trap_init};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn expect_illegal(_ev: Event, ctx: *mut Context) -> *mut Context {
    unsafe { if (*ctx).mcause == 2 { halt(0) } else { halt(1) } }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(expect_illegal);
    unsafe {
        // stimecmp (0x14d) 未实现：不得静默返回 0。
        core::arch::asm!("csrr t0, 0x14d", out("t0") _);
    }
    halt(1)
}
