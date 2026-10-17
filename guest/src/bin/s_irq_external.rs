//! S-mode 外部中断：UART RX → PLIC ctx1 → S external interrupt。

#![no_std]
#![no_main]

use rremu_am::{
    Event, SContext, enter_supervisor, halt, pmp_allow_all, trap_init, trap_init_s,
};
use rremu_guest::supervisor_exit_handler;

const UART_BASE: u32 = 0x1000_0000;
const PLIC_BASE: u32 = 0x0c00_0000;
const PLIC_ENABLE_S: u32 = PLIC_BASE + 0x2000 + 0x80;
const PLIC_PRIO10: u32 = PLIC_BASE + 10 * 4;
const PLIC_THRESHOLD_S: u32 = PLIC_BASE + 0x200_000 + 0x1000;
const PLIC_CLAIM_S: u32 = PLIC_BASE + 0x200_000 + 0x1000 + 4;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

fn wr(addr: u32, value: u32) {
    unsafe { core::ptr::write_volatile(addr as *mut u32, value) };
}

fn rd(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

fn wr8(addr: u32, value: u8) {
    unsafe { core::ptr::write_volatile(addr as *mut u8, value) };
}

extern "C" fn s_handler(ev: Event, _ctx: *mut SContext) -> *mut SContext {
    if ev == Event::IrqExternal {
        let source = rd(PLIC_CLAIM_S);
        if source == 10 {
            let _ = rd(UART_BASE); // 消费 RX 字节，撤销 UART IRQ
            wr(PLIC_CLAIM_S, 10); // complete
            halt(0)
        } else {
            halt(1)
        }
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

    // UART RX 中断使能 + PLIC S context(source 10)配置。
    wr8(UART_BASE + 1, 1); // IER.RX
    wr(PLIC_PRIO10, 1);
    wr(PLIC_THRESHOLD_S, 0);
    wr(PLIC_ENABLE_S, 1 << 10);

    unsafe {
        core::arch::asm!(
            "li t0, {deleg}",
            "csrw mideleg, t0",
            "li t0, {sie}",
            "csrw mie, t0",
            "li t0, {msie}",
            "csrs mstatus, t0",
            deleg = const (1u32 << 9),
            sie = const (1u32 << 9),
            msie = const (1u32 << 1),
            out("t0") _,
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
