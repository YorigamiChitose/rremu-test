//! ISA 测试：逻辑运算与移位（and/or/xor/not/sll/srl/sra）。
//!
//! 操作数经 volatile 读取，避免 host 常量折叠，保证指令真实执行。

#![no_std]
#![no_main]

use core::ptr::{addr_of, read_volatile};

static mut A: u32 = 0xF0;
static mut B: u32 = 0x0F;
static mut S: u32 = 1;
static mut X: u32 = 0x80;
static mut N: i32 = -8;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let (a, b, s, x, n) = unsafe {
        (
            read_volatile(addr_of!(A)),
            read_volatile(addr_of!(B)),
            read_volatile(addr_of!(S)),
            read_volatile(addr_of!(X)),
            read_volatile(addr_of!(N)),
        )
    };

    if a & b != 0x00 {
        rremu_am::halt(1);
    }
    if a | b != 0xFF {
        rremu_am::halt(1);
    }
    if a ^ b != 0xFF {
        rremu_am::halt(1);
    }
    if !a != 0xFFFF_FF0F {
        rremu_am::halt(1);
    }
    if s << 4 != 0x10 {
        rremu_am::halt(1);
    }
    if x >> 4 != 0x08 {
        rremu_am::halt(1);
    }
    if (n >> 1) != -4 {
        rremu_am::halt(1);
    }

    rremu_am::halt(0)
}
