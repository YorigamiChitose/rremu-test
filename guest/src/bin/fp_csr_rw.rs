//! FP e2e：fcsr / fflags / frm 别名、写掩码与 sticky flags。
#![no_std]
#![no_main]

use rremu_am::fp::{
    FFLAGS_INEXACT, FS_CLEAN, FS_DIRTY, clear_fflags, read_fcsr, read_fflags, read_frm, read_fs,
    set_fflags, write_fcsr, write_fflags, write_frm, write_fs,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    clear_fflags();

    write_fcsr(0xe5);
    if read_fcsr() != 0xe5 || read_fflags() != 0x05 || read_frm() != 7 {
        rremu_am::halt(10);
    }
    if read_fs() != FS_DIRTY {
        rremu_am::halt(11);
    }

    write_fflags(0x12);
    if read_fcsr() != 0xf2 {
        rremu_am::halt(12);
    }
    write_frm(2);
    if read_fcsr() != 0x52 || read_fflags() != 0x12 || read_frm() != 2 {
        rremu_am::halt(13);
    }

    set_fflags(FFLAGS_INEXACT);
    if read_fflags() != 0x13 {
        rremu_am::halt(14);
    }
    clear_fflags();
    if read_fflags() != 0 {
        rremu_am::halt(15);
    }

    write_fcsr(u32::MAX);
    if read_fcsr() != 0xff || read_fflags() != 0x1f || read_frm() != 7 {
        rremu_am::halt(16);
    }

    rremu_am::halt(0)
}
