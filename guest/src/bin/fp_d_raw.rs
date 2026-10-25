//! FP e2e：D 指令冒烟（原始 `.word` 编码）。
//!
//! 覆盖 fld/fsd、fadd/fsub/fmul/fdiv/fsqrt.d、fcvt.d.s、
//! fcvt.s.d、fcvt.w.d 与 fcvt.d.w。guest target 是 imafc，没有 D 代码生成，
//! 因此这里直接嵌入 R4/OP-FP 指令字。

#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, read_volatile, write_volatile};
use rremu_am::fp::{
    FFLAGS_INEXACT, FS_CLEAN, FS_DIRTY, clear_fflags, read_fflags, read_fs, write_fs,
};

static mut BUF: [u64; 10] = [0; 10];

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    clear_fflags();
    write_fs(FS_CLEAN);

    let p = unsafe { addr_of_mut!(BUF[0]) };
    unsafe {
        write_volatile(p, 0x3ff8_0000_0000_0000); // 1.5
        write_volatile(p.add(1), 0x4004_0000_0000_0000); // 2.5
    }

    let sd_result: u32;
    let wd_result: u32;
    unsafe {
        core::arch::asm!(
            ".word 0x00053507", // fld f10, 0(a0)
            ".word 0x00853587", // fld f11, 8(a0)
            ".word 0x00a53827", // fsd f10, 16(a0)
            ".word 0x02b57653", // fadd.d f12, f10, f11
            ".word 0x00c53c27", // fsd f12, 24(a0)
            ".word 0x0ab576d3", // fsub.d f13, f10, f11
            ".word 0x02d53027", // fsd f13, 32(a0)
            ".word 0x12b57753", // fmul.d f14, f10, f11
            ".word 0x02e53427", // fsd f14, 40(a0)
            ".word 0x1ab577d3", // fdiv.d f15, f10, f11
            ".word 0x02f53827", // fsd f15, 48(a0)
            ".word 0x5a05f853", // fsqrt.d f16, f11
            ".word 0x03053c27", // fsd f16, 56(a0)
            "fmv.w.x f0, {one_five_f32}",
            ".word 0x420008d3", // fcvt.d.s f17, f0
            ".word 0x05153027", // fsd f17, 64(a0)
            ".word 0x4018f053", // fcvt.s.d f0, f17
            "fmv.x.w {sd_result}, f0",
            ".word 0xc2050e53", // fcvt.w.d t3, f10, rne
            "mv {wd_result}, t3",
            ".word 0xd20688d3", // fcvt.d.w f17, a3
            ".word 0x05153427", // fsd f17, 72(a0)
            in("a0") p,
            in("a3") 7u32,
            one_five_f32 = in(reg) 0x3fc0_0000u32,
            sd_result = out(reg) sd_result,
            wd_result = out(reg) wd_result,
            out("f0") _, out("f10") _, out("f11") _, out("f12") _, out("f13") _,
            out("f14") _, out("f15") _, out("f16") _, out("f17") _,
            out("t3") _,
            options(nostack),
        );
    }

    if unsafe { read_volatile(p.add(2)) } != 0x3ff8_0000_0000_0000 {
        rremu_am::halt(10);
    }
    if unsafe { read_volatile(p.add(3)) } != 0x4010_0000_0000_0000 {
        rremu_am::halt(11);
    }
    if unsafe { read_volatile(p.add(4)) } != 0xbff0_0000_0000_0000 {
        rremu_am::halt(12);
    }
    if unsafe { read_volatile(p.add(5)) } != 0x400e_0000_0000_0000 {
        rremu_am::halt(13);
    }
    if unsafe { read_volatile(p.add(6)) } != 0x3fe3_3333_3333_3333 {
        rremu_am::halt(14);
    }
    if unsafe { read_volatile(p.add(7)) } != 0x3ff9_4c58_3ada_5b53 {
        rremu_am::halt(15);
    }
    if unsafe { read_volatile(p.add(8)) } != 0x3ff8_0000_0000_0000 {
        rremu_am::halt(16);
    }
    if unsafe { read_volatile(p.add(9)) } != 0x401c_0000_0000_0000 {
        rremu_am::halt(17);
    }
    if sd_result != 0x3fc0_0000 {
        rremu_am::halt(18);
    }
    if wd_result != 2 {
        rremu_am::halt(19);
    }
    if read_fflags() != FFLAGS_INEXACT {
        rremu_am::halt(20);
    }
    if read_fs() != FS_DIRTY {
        rremu_am::halt(21);
    }

    rremu_am::halt(0)
}
