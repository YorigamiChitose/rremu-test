//! S-mode 定时器中断：M 注入 STIP 并委派给 S，S handler 处理后退出。

#![no_std]
#![no_main]

use rremu_am::{
    Event, SContext, enter_supervisor, halt, pmp_allow_all, trap_init, trap_init_s,
};
use rremu_guest::supervisor_exit_handler;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn s_handler(ev: Event, _ctx: *mut SContext) -> *mut SContext {
    if ev == Event::IrqTimer {
        unsafe {
            core::arch::asm!("li t0, {stip}", "csrc sip, t0", stip = const (1u32 << 5), out("t0") _);
        }
        halt(0)
    } else {
        halt(1)
    }
}

extern "C" fn s_entry() -> ! {
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
            "li t0, {msie}",
            "csrs mstatus, t0",
            "li t0, {stip}",
            "csrw mip, t0",
            deleg = const (1u32 << 5),
            sie = const (1u32 << 5),
            msie = const (1u32 << 1),
            stip = const (1u32 << 5),
            out("t0") _,
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
