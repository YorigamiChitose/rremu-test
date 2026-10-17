//! S-mode 测试：M → S，设置 SPP 后 `sret` 仍回到 S。
//!
//! 进入 S 后把 `sepc` 指向第二段 S 代码，并置 `sstatus.SPP=1`；
//! `sret` 后 SPP 应清零，且代码仍在 S-mode 执行（通过 S-mode `halt` 桥接退出）。

#![no_std]
#![no_main]

use rremu_am::{enter_supervisor, halt, trap_init};
use rremu_guest::supervisor_exit_handler;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn s_after_sret() -> ! {
    let sstatus: u32;
    unsafe { core::arch::asm!("csrr {sstatus}, sstatus", sstatus = out(reg) sstatus) };
    if sstatus & (1 << 8) == 0 {
        halt(0)
    } else {
        halt(1)
    }
}

extern "C" fn s_entry() -> ! {
    unsafe {
        core::arch::asm!(
            "la t0, {after}",
            "csrw sepc, t0",
            "li t0, {spp}",
            "csrs sstatus, t0",
            "sret",
            after = sym s_after_sret,
            spp = const (1u32 << 8),
            clobber_abi("C"),
            options(noreturn),
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(supervisor_exit_handler);
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
