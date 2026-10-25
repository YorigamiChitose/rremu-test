//! FP e2e：feq/flt/fle.s 与 sNaN invalid 语义。
#![no_std]
#![no_main]

use rremu_am::fp::{
    FFLAGS_INVALID, FS_CLEAN, FS_DIRTY, clear_fflags, read_fflags, read_fs, write_fs,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    unsafe {
        core::arch::asm!(
            "fmv.w.x f10, {one}",
            "fmv.w.x f11, {one}",
            "fmv.w.x f12, {two}",
            "fmv.w.x f13, {qnan}",
            "fmv.w.x f14, {snan}",
            "fmv.w.x f15, {neg_zero}",
            "fmv.w.x f16, {pos_zero}",
            one = in(reg) 0x3f80_0000u32,
            two = in(reg) 0x4000_0000u32,
            qnan = in(reg) 0x7fc0_0000u32,
            snan = in(reg) 0x7f80_0001u32,
            neg_zero = in(reg) 0x8000_0000u32,
            pos_zero = in(reg) 0x0000_0000u32,
            out("f10") _, out("f11") _, out("f12") _, out("f13") _,
            out("f14") _, out("f15") _, out("f16") _,
            options(nostack),
        );
    }
    clear_fflags();
    write_fs(FS_CLEAN);

    let eq: u32;
    let lt: u32;
    let le: u32;
    let zero_eq: u32;
    let nan_eq: u32;
    unsafe {
        core::arch::asm!(
            "feq.s {eq}, f10, f11",
            "flt.s {lt}, f10, f12",
            "fle.s {le}, f12, f10",
            "feq.s {zero_eq}, f15, f16",
            "feq.s {nan_eq}, f13, f13",
            eq = out(reg) eq,
            lt = out(reg) lt,
            le = out(reg) le,
            zero_eq = out(reg) zero_eq,
            nan_eq = out(reg) nan_eq,
            out("f10") _, out("f11") _, out("f12") _, out("f13") _,
            out("f15") _, out("f16") _,
            options(nostack),
        );
    }

    if eq != 1 || lt != 1 || le != 0 || zero_eq != 1 || nan_eq != 0 {
        rremu_am::halt(10);
    }
    if read_fflags() != 0 {
        rremu_am::halt(11);
    }
    if read_fs() != FS_CLEAN {
        rremu_am::halt(12);
    }

    let snan_lt: u32;
    unsafe {
        core::arch::asm!(
            "flt.s {snan_lt}, f14, f10",
            snan_lt = out(reg) snan_lt,
            out("f14") _, out("f10") _,
            options(nostack),
        );
    }
    if snan_lt != 0 {
        rremu_am::halt(13);
    }
    if read_fflags() != FFLAGS_INVALID {
        rremu_am::halt(14);
    }
    if read_fs() != FS_DIRTY {
        rremu_am::halt(15);
    }

    rremu_am::halt(0)
}
