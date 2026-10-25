//! S → U 测试：`sret` 以 SPP=0 返回 U，U-mode ecall 触发 cause 8。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, enter_supervisor, halt, pmp_allow_all, trap_init};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn expect_user_ecall(_ev: Event, ctx: *mut Context) -> *mut Context {
    unsafe { if (*ctx).mcause == 8 { halt(0) } else { halt(1) } }
}

extern "C" fn u_entry() -> ! {
    unsafe {
        core::arch::asm!("ecall", in("a7") 0u32, options(noreturn));
    }
}

extern "C" fn s_entry() -> ! {
    unsafe {
        core::arch::asm!(
            "la t0, {u_entry}",
            "csrw sepc, t0",
            "li t0, 0x100",       // SPP
            "csrc sstatus, t0",   // SPP=0 → sret 回 U
            "sret",
            u_entry = sym u_entry,
            clobber_abi("C"),
            options(noreturn),
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    pmp_allow_all();
    trap_init(expect_user_ecall);
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
