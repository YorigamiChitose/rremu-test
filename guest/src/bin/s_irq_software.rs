//! S-mode 软件中断：M 注入 SSIP 并委派给 S，S handler 处理后退出。

#![no_std]
#![no_main]

use rremu_am::{Event, SContext, enter_supervisor, halt, pmp_allow_all, trap_init, trap_init_s};
use rremu_guest::supervisor_exit_handler;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn s_handler(ev: Event, _ctx: *mut SContext) -> *mut SContext {
    if ev == Event::IrqSoftware {
        unsafe { core::arch::asm!("csrw sip, zero") };
        halt(0)
    } else {
        halt(1)
    }
}

extern "C" fn s_entry() -> ! {
    // 若中断未被注入，走这里失败。
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    pmp_allow_all();
    trap_init(supervisor_exit_handler);
    trap_init_s(s_handler);
    unsafe {
        core::arch::asm!(
            "li t0, {deleg}",
            "csrw mideleg, t0",
            "li t0, {sie}",
            "csrw mie, t0",
            "csrs mstatus, t0",
            "li t0, {ssip}",
            "csrw mip, t0",
            deleg = const (1u32 << 1),
            sie = const (1u32 << 1),
            ssip = const (1u32 << 1),
            out("t0") _,
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
