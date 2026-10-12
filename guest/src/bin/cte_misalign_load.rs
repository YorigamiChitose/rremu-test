//! CTE 测试：MISALIGN 事件（lw 从未对齐地址）。
#![no_std]
#![no_main]
use rremu_am::{Context, Event, trap_init, halt};
static mut FLAG: bool = false;
extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::Misalign {
        unsafe {
            FLAG = true;
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
    // 在栈上取一个未 4 字节对齐的地址做 lw
    let mut buf: [u8; 16] = [0; 16];
    let addr = buf.as_mut_ptr() as u32 + 1; // 奇地址
    unsafe {
        core::arch::asm!(
            "lw zero, 0({addr})",
            addr = in(reg) addr,
            options(nostack),
        );
    }
    if unsafe { FLAG } { halt(0) } else { halt(1) }
}
