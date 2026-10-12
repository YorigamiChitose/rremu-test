//! CTE 测试：陷阱入口/出口寄存器保存/恢复正确性。
//!
//! guest 用已知值填充 t0-t6/a0-a6 后 ecall(Yield)，
//! handler 验证 Context.gpr 中保存的值一致，设 FLAG，
//! 推进 mepc 后返回。mret 后 guest 检查 FLAG。

#![no_std]
#![no_main]
use rremu_am::{Context, Event, trap_init, halt};

static mut CHECK_OK: bool = false;

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::Yield {
        unsafe {
            let g = &(*ctx).gpr;
            let ok = g[5]==0xA0 && g[6]==0xA1 && g[7]==0xA2
                && g[28]==0xA3 && g[29]==0xA4 && g[30]==0xA5 && g[31]==0xA6
                && g[10]==0xA7 && g[11]==0xA8 && g[12]==0xA9
                && g[13]==0xAA && g[14]==0xAB && g[15]==0xAC && g[16]==0xAD;
            CHECK_OK = ok;
            let mepc = (*ctx).mepc;
            let half = core::ptr::read_volatile(mepc as *const u16);
            let len = if half & 0x3 == 0x3 { 4 } else { 2 };
            (*ctx).mepc = mepc.wrapping_add(len);
        }
    }
    ctx
}

#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(handler);
    unsafe {
        core::arch::asm!(
            "li t0, 0xA0",
            "li t1, 0xA1",
            "li t2, 0xA2",
            "li t3, 0xA3",
            "li t4, 0xA4",
            "li t5, 0xA5",
            "li t6, 0xA6",
            "li a0, 0xA7",
            "li a1, 0xA8",
            "li a2, 0xA9",
            "li a3, 0xAA",
            "li a4, 0xAB",
            "li a5, 0xAC",
            "li a6, 0xAD",
            "li a7, -1",
            "ecall",
            out("t0") _, out("t1") _, out("t2") _,
            out("t3") _, out("t4") _, out("t5") _, out("t6") _,
            out("a0") _, out("a1") _, out("a2") _,
            out("a3") _, out("a4") _, out("a5") _, out("a6") _,
            out("a7") _,
            options(nostack),
        );
    }
    if unsafe { CHECK_OK } { halt(0) } else { halt(1) }
}
