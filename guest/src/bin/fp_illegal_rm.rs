//! FP e2e：静态/动态保留舍入模式触发 IllInsn，且不改变 FS。
#![no_std]
#![no_main]

use core::ptr::read_volatile;
use rremu_am::fp::{FS_CLEAN, read_frm, read_fs, write_frm, write_fs};
use rremu_am::{Context, Event, halt, trap_init};

static mut CAUGHT: u32 = 0;

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev != Event::IllInsn {
        halt(1);
    }
    unsafe {
        CAUGHT += 1;
        let mepc = (*ctx).mepc;
        let half = read_volatile(mepc as *const u16);
        let len = if half & 0x3 == 0x3 { 4 } else { 2 };
        (*ctx).mepc = mepc.wrapping_add(len);
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
            ".word 0x00005053", // fadd.s rm=5
            ".word 0x00006053", // fadd.s rm=6
            options(nostack),
        );
    }
    if read_fs() != FS_CLEAN {
        halt(10);
    }

    write_frm(5);
    write_fs(FS_CLEAN);
    unsafe {
        core::arch::asm!(
            ".word 0x00007053", // fadd.s rm=dyn, frm=5
            options(nostack),
        );
    }

    if unsafe { CAUGHT } != 3 {
        halt(11);
    }
    if read_fs() != FS_CLEAN {
        halt(12);
    }
    if read_frm() != 5 {
        halt(13);
    }

    halt(0)
}
