//! FP e2e：fadd/fsub/fmul/fdiv/fsqrt.s 的结果、flags 与 FS。
#![no_std]
#![no_main]

use rremu_am::fp::{
    FFLAGS_INEXACT, FS_CLEAN, FS_DIRTY, clear_fflags, read_fflags, read_fs, write_fs,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    clear_fflags();

    let add: u32;
    let sub: u32;
    let mul: u32;
    let div: u32;
    let sqrt: u32;
    unsafe {
        core::arch::asm!(
            "fmv.w.x f10, {one}",
            "fmv.w.x f11, {two}",
            "fmv.w.x f12, {three}",
            "fadd.s f13, f10, f11",
            "fsub.s f14, f10, f11",
            "fmul.s f15, f10, f11",
            "fdiv.s f16, f10, f12",
            "fsqrt.s f17, f11",
            "fmv.x.w {add}, f13",
            "fmv.x.w {sub}, f14",
            "fmv.x.w {mul}, f15",
            "fmv.x.w {div}, f16",
            "fmv.x.w {sqrt}, f17",
            one = in(reg) 0x3f80_0000u32,
            two = in(reg) 0x4000_0000u32,
            three = in(reg) 0x4040_0000u32,
            add = out(reg) add,
            sub = out(reg) sub,
            mul = out(reg) mul,
            div = out(reg) div,
            sqrt = out(reg) sqrt,
            out("f10") _, out("f11") _, out("f12") _, out("f13") _,
            out("f14") _, out("f15") _, out("f16") _, out("f17") _,
            options(nostack),
        );
    }

    if add != 0x4040_0000 {
        rremu_am::halt(10);
    }
    if sub != 0xbf80_0000 {
        rremu_am::halt(11);
    }
    if mul != 0x4000_0000 {
        rremu_am::halt(12);
    }
    if div != 0x3eaa_aaab {
        rremu_am::halt(13);
    }
    if sqrt != 0x3fb5_04f3 {
        rremu_am::halt(14);
    }
    if read_fflags() != FFLAGS_INEXACT {
        rremu_am::halt(15);
    }
    if read_fs() != FS_DIRTY {
        rremu_am::halt(16);
    }

    rremu_am::halt(0)
}
