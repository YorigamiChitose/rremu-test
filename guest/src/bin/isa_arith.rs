//! ISA 测试：基础算术 + M 扩展（add/sub/mul/div/rem）。
//!
//! 操作数经 volatile 读取，编译器无法在 host 折叠，
//! 保证生成真实的算术/M 指令并由模拟器执行。

#![no_std]
#![no_main]

use core::ptr::{addr_of, read_volatile};

static mut A: u32 = 6;
static mut B: u32 = 3;
static mut C: u32 = 7;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let (a, b, c) = unsafe {
        (
            read_volatile(addr_of!(A)),
            read_volatile(addr_of!(B)),
            read_volatile(addr_of!(C)),
        )
    };

    if a + b != 9 { rremu_am::halt(1); }
    if a - b != 3 { rremu_am::halt(1); }
    if a * b != 18 { rremu_am::halt(1); }
    if a / b != 2 { rremu_am::halt(1); }
    if a % b != 0 { rremu_am::halt(1); }
    if c * c != 49 { rremu_am::halt(1); }

    rremu_am::halt(0)
}
