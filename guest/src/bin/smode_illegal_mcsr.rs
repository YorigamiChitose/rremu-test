//! S-mode 测试：访问 M-mode CSR 触发 illegal instruction。
//!
//! S 代码执行 `csrr mstatus`，预期陷入 M-mode 且 mcause=2；
//! 测试专用 handler 校验 cause 后以 0 退出。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, enter_supervisor, halt, pmp_allow_all, trap_init};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn expect_illegal(_ev: Event, ctx: *mut Context) -> *mut Context {
    unsafe { if (*ctx).mcause == 2 { halt(0) } else { halt(1) } }
}

extern "C" fn s_entry() -> ! {
    unsafe {
        core::arch::asm!("csrr t0, mstatus", clobber_abi("C"), options(noreturn));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    pmp_allow_all();
    trap_init(expect_illegal);
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
