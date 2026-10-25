//! FP e2e：fcvt.w/wu.s 与 fcvt.s.w/wu 的结果与 flags。
#![no_std]
#![no_main]

use rremu_am::fp::{
    FFLAGS_INEXACT, FFLAGS_INVALID, FS_CLEAN, FS_DIRTY, clear_fflags, read_fflags, read_fs,
    write_frm, write_fs,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    clear_fflags();
    write_frm(0);

    let w1: u32;
    let wu1: u32;
    let w2: u32;
    let s1: u32;
    let s2: u32;
    unsafe {
        core::arch::asm!(
            "fmv.w.x f10, {one_point_five}",
            "fmv.w.x f11, {neg_half}",
            "fmv.w.x f12, {two_pow_31}",
            "fcvt.w.s {w1}, f10, rne",
            "fcvt.wu.s {wu1}, f11, rne",
            "fcvt.w.s {w2}, f12, rne",
            "fcvt.s.w f13, {i1}, rne",
            "fmv.x.w {s1}, f13",
            "fcvt.s.wu f13, {u1}, rne",
            "fmv.x.w {s2}, f13",
            one_point_five = in(reg) 0x3fc0_0000u32,
            neg_half = in(reg) 0xbf00_0000u32,
            two_pow_31 = in(reg) 0x4f00_0000u32,
            i1 = in(reg) 0x0100_0001u32,
            u1 = in(reg) 0xffff_ffffu32,
            w1 = out(reg) w1,
            wu1 = out(reg) wu1,
            w2 = out(reg) w2,
            s1 = out(reg) s1,
            s2 = out(reg) s2,
            out("f10") _, out("f11") _, out("f12") _, out("f13") _,
            options(nostack),
        );
    }

    if w1 != 2 {
        rremu_am::halt(10);
    }
    if wu1 != 0 {
        rremu_am::halt(11);
    }
    if w2 != 0x7fff_ffff {
        rremu_am::halt(12);
    }
    if s1 != 0x4b80_0000 {
        rremu_am::halt(13);
    }
    if s2 != 0x4f80_0000 {
        rremu_am::halt(14);
    }
    if read_fflags() != (FFLAGS_INVALID | FFLAGS_INEXACT) {
        rremu_am::halt(15);
    }
    if read_fs() != FS_DIRTY {
        rremu_am::halt(16);
    }

    rremu_am::halt(0)
}
