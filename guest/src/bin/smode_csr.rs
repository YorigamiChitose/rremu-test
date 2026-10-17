//! S-mode 测试：S-CSR 读写与 sstatus 视图。
//!
//! 重点验证 S-mode 下 `sscratch` 可读写、`sstatus.SIE` 可置位并读回。

#![no_std]
#![no_main]

use rremu_am::{enter_supervisor, halt, trap_init};
use rremu_guest::supervisor_exit_handler;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn s_entry() -> ! {
    let scratch: u32;
    let sstatus: u32;
    unsafe {
        core::arch::asm!(
            "csrw sscratch, {v}",
            "csrr {scratch}, sscratch",
            "csrs sstatus, {sie}",
            "csrr {sstatus}, sstatus",
            v = in(reg) 0x5A5A_1234u32,
            sie = in(reg) 1u32 << 1,
            scratch = out(reg) scratch,
            sstatus = out(reg) sstatus,
        );
    }
    if scratch == 0x5A5A_1234 && sstatus & (1 << 1) != 0 {
        halt(0)
    } else {
        halt(1)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(supervisor_exit_handler);
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
