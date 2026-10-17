//! S-mode 委派测试：`medeleg` 把 S-mode ecall 交给 AM 的 S 陷阱入口。
//!
//! S handler 校验 `scause=9` 后执行非法指令，让 M handler 观察结果并退出。
//! 同时验证委派路径、`scause` 与 `__am_asm_strap` / `SContext` 布局。

#![no_std]
#![no_main]

use rremu_am::{
    Context, Event, SContext, enter_supervisor, halt, pmp_allow_all, trap_init, trap_init_s,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

/// S-mode handler：校验 cause，然后主动触发未委派的 illegal 回到 M。
extern "C" fn s_handler(_ev: Event, ctx: *mut SContext) -> *mut SContext {
    unsafe {
        if (*ctx).scause == 9 {
            (*ctx).gpr[10] = 0;
        } else {
            (*ctx).gpr[10] = 1;
        }
        core::arch::asm!("unimp", options(noreturn));
    }
}

/// M handler：S handler 的 `unimp` 应带来 mcause=2，a0=0 表示 scause 正确。
extern "C" fn m_handler(_ev: Event, ctx: *mut Context) -> *mut Context {
    unsafe {
        if (*ctx).mcause == 2 && (*ctx).gpr[10] == 0 {
            halt(0)
        } else {
            halt(1)
        }
    }
}

extern "C" fn s_entry() -> ! {
    unsafe {
        core::arch::asm!("ecall", in("a7") rremu_am::EXIT_MAGIC, options(noreturn));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    pmp_allow_all();
    trap_init(m_handler);
    trap_init_s(s_handler);
    unsafe {
        core::arch::asm!(
            "li t0, {ecall_s}",
            "csrw medeleg, t0",
            ecall_s = const (1u32 << 9),
            out("t0") _,
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
