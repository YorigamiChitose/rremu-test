//! ISA 测试：A 扩展原子指令（amoadd/amoswap/amoxor/amoand/amoor/
//! amomin/amomax/amominu/amomaxu）+ fence.i。
//!
//! 内联 asm 明确指定指令，避免编译器优化；操作 volatile 静态单元。

#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, read_volatile, write_volatile};

static mut CELL: u32 = 0;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        let p = addr_of_mut!(CELL);

        // amoadd.w：5 + 3 → old=5, CELL=8
        write_volatile(p, 5);
        let old: u32;
        core::arch::asm!(
            "amoadd.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 3u32,
            old = out(reg) old,
        );
        if old != 5 || read_volatile(p) != 8 {
            rremu_am::halt(1);
        }

        // amoswap.w：换 0xABC → old=8, CELL=0xABC
        let old: u32;
        core::arch::asm!(
            "amoswap.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0xABC,
            old = out(reg) old,
        );
        if old != 8 || read_volatile(p) != 0xABC {
            rremu_am::halt(1);
        }

        // amoxor.w：0xFF ^ 0x0F → old=0xFF, CELL=0xF0
        write_volatile(p, 0xFF);
        let old: u32;
        core::arch::asm!(
            "amoxor.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0x0F,
            old = out(reg) old,
        );
        if old != 0xFF || read_volatile(p) != 0xF0 {
            rremu_am::halt(1);
        }

        // amoand.w：0xF0 & 0x0F → old=0xF0, CELL=0
        let old: u32;
        core::arch::asm!(
            "amoand.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0x0F,
            old = out(reg) old,
        );
        if old != 0xF0 || read_volatile(p) != 0 {
            rremu_am::halt(1);
        }

        // amoor.w：0 | 0x0F → old=0, CELL=0x0F
        let old: u32;
        core::arch::asm!(
            "amoor.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0x0F,
            old = out(reg) old,
        );
        if old != 0 || read_volatile(p) != 0x0F {
            rremu_am::halt(1);
        }

        // amomin.w（有符号）：min(-1, 0) = -1 → old=-1, CELL=-1
        write_volatile(p, 0xFFFF_FFFFu32);
        let old: u32;
        core::arch::asm!(
            "amomin.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0u32,
            old = out(reg) old,
        );
        if old != 0xFFFF_FFFF || read_volatile(p) != 0xFFFF_FFFF {
            rremu_am::halt(1);
        }

        // amomax.w（有符号）：max(-1, 0) = 0 → old=-1, CELL=0
        let old: u32;
        core::arch::asm!(
            "amomax.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0u32,
            old = out(reg) old,
        );
        if old != 0xFFFF_FFFF || read_volatile(p) != 0 {
            rremu_am::halt(1);
        }

        // amominu.w（无符号）：min(0xFFFFFFFF, 5) = 5 → old=0xFFFFFFFF, CELL=5
        write_volatile(p, 0xFFFF_FFFFu32);
        let old: u32;
        core::arch::asm!(
            "amominu.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 5u32,
            old = out(reg) old,
        );
        if old != 0xFFFF_FFFF || read_volatile(p) != 5 {
            rremu_am::halt(1);
        }

        // amomaxu.w（无符号）：max(5, 100) = 100 → old=5, CELL=100
        let old: u32;
        core::arch::asm!(
            "amomaxu.w {old}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 100u32,
            old = out(reg) old,
        );
        if old != 5 || read_volatile(p) != 100 {
            rremu_am::halt(1);
        }

        // fence.i：NOP（无指令缓存），执行后应继续
        core::arch::asm!("fence.i");
    }

    rremu_am::halt(0)
}
