//! S-mode 委派测试：`medeleg` 把 S-mode ecall 交给 S 的 `stvec`。
//!
//! S handler 校验 `scause=9` 后执行非法指令，让 M handler 观察结果并退出。
//! 这样同时验证委派路径、`scause` 以及 S-mode handler 的进入地址。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, enter_supervisor, halt, trap_init};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

/// S-mode 陷阱入口：只验证 cause，然后主动触发未委派的 illegal 回到 M。
extern "C" fn s_trap() -> ! {
    unsafe {
        core::arch::asm!(
            "csrr t0, scause",
            "li t1, 9",
            "li a0, 1",
            "bne t0, t1, 2f",
            "li a0, 0",
            "2:",
            "unimp",
            clobber_abi("C"),
            options(noreturn),
        );
    }
}

/// M handler：S trap handler 的 `unimp` 应带来 mcause=2，a0=0 表示 scause 正确。
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
    trap_init(m_handler);
    unsafe {
        core::arch::asm!(
            "la t0, {s_trap}",
            "csrw stvec, t0",
            "li t0, {ecall_s}",
            "csrw medeleg, t0",
            s_trap = sym s_trap,
            ecall_s = const (1u32 << 9),
            out("t0") _,
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
