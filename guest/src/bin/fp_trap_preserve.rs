//! FP e2e：陷阱入口/出口不破坏 FPR 与 FS。
#![no_std]
#![no_main]

use rremu_am::fp::{FS_CLEAN, FS_DIRTY, read_fs, write_fs};
use rremu_am::{Context, Event, halt, trap_init};

static mut CHECK_OK: bool = false;

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::Yield {
        let f10: u32;
        let f11: u32;
        let f12: u32;
        unsafe {
            core::arch::asm!(
                "fmv.x.w {f10}, f10",
                "fmv.x.w {f11}, f11",
                "fmv.x.w {f12}, f12",
                f10 = out(reg) f10,
                f11 = out(reg) f11,
                f12 = out(reg) f12,
                options(nostack),
            );
            CHECK_OK = f10 == 0x3f80_0000
                && f11 == 0x4000_0000
                && f12 == 0x7fc0_1234
                && read_fs() == FS_DIRTY;
            (*ctx).mepc = (*ctx).mepc.wrapping_add(4);
        }
    }
    ctx
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    trap_init(handler);
    unsafe {
        core::arch::asm!(
            "fmv.w.x f10, {one}",
            "fmv.w.x f11, {two}",
            "fmv.w.x f12, {qnan}",
            "li a7, -1",
            "ecall",
            one = in(reg) 0x3f80_0000u32,
            two = in(reg) 0x4000_0000u32,
            qnan = in(reg) 0x7fc0_1234u32,
            out("f10") _, out("f11") _, out("f12") _,
            out("a7") _,
            options(nostack),
        );
    }

    let f10: u32;
    let f11: u32;
    let f12: u32;
    unsafe {
        core::arch::asm!(
            "fmv.x.w {f10}, f10",
            "fmv.x.w {f11}, f11",
            "fmv.x.w {f12}, f12",
            f10 = out(reg) f10,
            f11 = out(reg) f11,
            f12 = out(reg) f12,
            options(nostack),
        );
    }

    if unsafe { CHECK_OK }
        && f10 == 0x3f80_0000
        && f11 == 0x4000_0000
        && f12 == 0x7fc0_1234
        && read_fs() == FS_DIRTY
    {
        halt(0)
    } else {
        halt(1)
    }
}
