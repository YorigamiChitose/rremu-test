//! ISA 测试：分支与函数调用（beq/bne/blt/bge/bltu + jal/jalr）。
//!
//! 操作数经 volatile 读取，避免 host 常量折叠；double 用 #[inline(never)]
//! 保证生成真实的函数调用（jal/jalr）。

#![no_std]
#![no_main]

use core::ptr::{addr_of, read_volatile};

static mut A: u32 = 5;
static mut B: u32 = 5;
static mut C: u32 = 3;
static mut X: i32 = -1;
static mut Y: i32 = 0;
static mut U: u32 = 0xFFFF_FFFF;
static mut V: u32 = 0;
static mut N: u32 = 21;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let (a, b, c, x, y, u, v, n) = unsafe {
        (
            read_volatile(addr_of!(A)),
            read_volatile(addr_of!(B)),
            read_volatile(addr_of!(C)),
            read_volatile(addr_of!(X)),
            read_volatile(addr_of!(Y)),
            read_volatile(addr_of!(U)),
            read_volatile(addr_of!(V)),
            read_volatile(addr_of!(N)),
        )
    };

    // beq
    if a == b {
    } else {
        rremu_am::halt(1);
    }

    // bne
    if a != c {
    } else {
        rremu_am::halt(1);
    }

    // blt（有符号 <）
    if x < y {
    } else {
        rremu_am::halt(1);
    }

    // bge（有符号 >=）
    if x >= y {
        rremu_am::halt(1);
    }

    // bltu（无符号 >，经 u > v 表达）
    if u > v {
    } else {
        rremu_am::halt(1);
    }

    // jal/jalr 经函数调用
    let result = double(n);
    if result != 42 {
        rremu_am::halt(1);
    }

    rremu_am::halt(0)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn double(x: u32) -> u32 {
    x * 2
}
