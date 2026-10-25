//! FP e2e：fmadd/fmsub/fnmadd/fnmsub.s 与单次舍入。
#![no_std]
#![no_main]

use rremu_am::fp::{FS_CLEAN, FS_DIRTY, clear_fflags, read_fflags, read_fs, write_fs};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    clear_fflags();

    let fmadd: u32;
    let fmsub: u32;
    let fnmsub: u32;
    let fnmadd: u32;
    let fused: u32;
    unsafe {
        core::arch::asm!(
            "fmv.w.x f10, {one}",
            "fmv.w.x f11, {two}",
            "fmv.w.x f12, {three}",
            "fmadd.s f13, f10, f11, f12",
            "fmv.x.w {fmadd}, f13",
            "fmsub.s f13, f10, f11, f12",
            "fmv.x.w {fmsub}, f13",
            "fnmsub.s f13, f10, f11, f12",
            "fmv.x.w {fnmsub}, f13",
            "fnmadd.s f13, f10, f11, f12",
            "fmv.x.w {fnmadd}, f13",
            "fmv.w.x f10, {a}",
            "fmv.w.x f11, {a}",
            "fmv.w.x f12, {c}",
            "fmadd.s f13, f10, f11, f12",
            "fmv.x.w {fused}, f13",
            one = in(reg) 0x3f80_0000u32,
            two = in(reg) 0x4000_0000u32,
            three = in(reg) 0x4040_0000u32,
            a = in(reg) 0x3f80_0001u32,
            c = in(reg) 0xbf80_0002u32,
            fmadd = out(reg) fmadd,
            fmsub = out(reg) fmsub,
            fnmsub = out(reg) fnmsub,
            fnmadd = out(reg) fnmadd,
            fused = out(reg) fused,
            out("f10") _, out("f11") _, out("f12") _, out("f13") _,
            options(nostack),
        );
    }

    if fmadd != 0x40a0_0000 {
        rremu_am::halt(10);
    }
    if fmsub != 0xbf80_0000 {
        rremu_am::halt(11);
    }
    if fnmsub != 0x3f80_0000 {
        rremu_am::halt(12);
    }
    if fnmadd != 0xc0a0_0000 {
        rremu_am::halt(13);
    }
    if fused != 0x2880_0000 {
        rremu_am::halt(14);
    }
    if read_fflags() != 0 {
        rremu_am::halt(15);
    }
    if read_fs() != FS_DIRTY {
        rremu_am::halt(16);
    }

    rremu_am::halt(0)
}
