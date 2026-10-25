//! FP e2e：flw/fsw 低 32 位读写与 NaN box。
#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, read_volatile};
use rremu_am::fp::{FS_CLEAN, write_fs};

static mut BUF: [u32; 4] = [0; 4];

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    let p = unsafe { addr_of_mut!(BUF[0]) };

    unsafe {
        core::arch::asm!(
            "fmv.w.x f1, {value}",
            "fsw f1, 0({p})",
            value = in(reg) 0x7fc0_1234u32,
            p = in(reg) p,
            out("f1") _,
            options(nostack),
        );
        if read_volatile(p) != 0x7fc0_1234 {
            rremu_am::halt(10);
        }

        let got: u32;
        core::arch::asm!(
            "flw f2, 0({p})",
            "fmv.x.w {got}, f2",
            p = in(reg) p,
            got = out(reg) got,
            out("f2") _,
            options(nostack),
        );
        if got != 0x7fc0_1234 {
            rremu_am::halt(11);
        }

        core::arch::asm!(
            "fmv.w.x f3, {value}",
            "fsw f3, 4({p})",
            value = in(reg) 0x3f80_0000u32,
            p = in(reg) p,
            out("f3") _,
            options(nostack),
        );
        if read_volatile(p.add(1)) != 0x3f80_0000 {
            rremu_am::halt(12);
        }
    }

    rremu_am::halt(0)
}
