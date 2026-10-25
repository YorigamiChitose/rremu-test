//! 边界测试：除零行为。
//!
//! RISC-V M 扩展规定除零不触发异常：
//! - div  by zero → 商 = 0xFFFFFFFF（全 1，即 -1）
//! - rem  by zero → 余数 = 被除数

#![no_std]
#![no_main]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let dividend: u32 = 42;
    let q: u32;
    let r: u32;
    unsafe {
        core::arch::asm!(
            "div {q}, {a}, zero",
            "rem {r}, {a}, zero",
            a = in(reg) dividend,
            q = out(reg) q,
            r = out(reg) r,
        );
    }
    // div by zero → -1 (0xFFFFFFFF), rem by zero → dividend
    if q == u32::MAX && r == dividend {
        rremu_am::halt(0)
    } else {
        rremu_am::halt(1)
    }
}
