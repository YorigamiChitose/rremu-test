//! PMP 端到端测试：S-mode 向被 PMP 拒绝的 RAM 地址存储触发 store access fault。

#![no_std]
#![no_main]

use rremu_am::{Context, Event, enter_supervisor, halt, trap_init};

const DENIED_ADDR: u32 = 0x8000_1000;
const RAM_BASE: u32 = 0x8000_0000;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn expect_store_fault(_ev: Event, ctx: *mut Context) -> *mut Context {
    unsafe { if (*ctx).mcause == 7 { halt(0) } else { halt(1) } }
}

extern "C" fn s_entry() -> ! {
    unsafe {
        // a0 = DENIED_ADDR；若 PMP 未拒绝，会继续执行到 halt(1)。
        core::arch::asm!("sw zero, 0(a0)");
    }
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(expect_store_fault);

    // entry0: NA4 @ DENIED_ADDR，无 R/W/X（最高优先级，拒绝）
    // entry1: NAPOT 覆盖 0x80000000..0x81000000，R/W/X（允许其余 RAM）
    let napot_ram = (RAM_BASE >> 2) | ((1 << 21) - 1);
    unsafe {
        core::arch::asm!(
            "li t0, {denied}",
            "csrw pmpaddr0, t0",
            "mv t0, {ram}",
            "csrw pmpaddr1, t0",
            "li t0, 0x1f10",       // entry1 R/W/X+NAPOT，entry0 NA4 无权限
            "csrw pmpcfg0, t0",
            denied = const (DENIED_ADDR >> 2),
            ram = in(reg) napot_ram,
            out("t0") _,
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, DENIED_ADDR, 0)
}
