//! FP e2e：FP 写寄存器后 FS=Dirty 且 mstatus.SD=1。
#![no_std]
#![no_main]

use rremu_am::fp::{FS_CLEAN, FS_DIRTY, read_fs, read_mstatus, write_fs};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    if read_fs() != FS_CLEAN {
        rremu_am::halt(10);
    }

    unsafe {
        core::arch::asm!("fmv.w.x f0, zero", out("f0") _, options(nostack));
    }

    if read_fs() != FS_DIRTY {
        rremu_am::halt(11);
    }
    if read_mstatus() & (1 << 31) == 0 {
        rremu_am::halt(12);
    }

    rremu_am::halt(0)
}
