//! FP e2e：S-mode 下 sstatus.FS 可写、SD 只读。
#![no_std]
#![no_main]

use rremu_am::fp::{FS_CLEAN, FS_DIRTY, write_fs};
use rremu_am::{enter_supervisor, halt, pmp_allow_all, trap_init};
use rremu_guest::supervisor_exit_handler;

const FS_MASK: u32 = 0b11 << 13;
const SD: u32 = 1 << 31;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn s_entry() -> ! {
    let before: u32;
    unsafe {
        core::arch::asm!("csrr {before}, sstatus", before = out(reg) before);
    }
    if before & FS_MASK != FS_CLEAN || before & SD != 0 {
        halt(10);
    }

    unsafe {
        core::arch::asm!(
            "csrs sstatus, {dirty}",
            dirty = in(reg) FS_DIRTY,
        );
    }
    let dirty: u32;
    unsafe {
        core::arch::asm!("csrr {dirty}, sstatus", dirty = out(reg) dirty);
    }
    if dirty & FS_MASK != FS_DIRTY || dirty & SD == 0 {
        halt(11);
    }

    let base: u32;
    unsafe {
        core::arch::asm!("csrr {base}, sstatus", base = out(reg) base);
    }
    let value = (base & !(FS_MASK | SD)) | FS_CLEAN | SD;
    unsafe {
        core::arch::asm!(
            "csrw sstatus, {value}",
            value = in(reg) value,
        );
    }
    let clean: u32;
    unsafe {
        core::arch::asm!("csrr {clean}, sstatus", clean = out(reg) clean);
    }
    if clean & FS_MASK != FS_CLEAN || clean & SD != 0 {
        halt(12);
    }

    halt(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    pmp_allow_all();
    write_fs(FS_CLEAN);
    trap_init(supervisor_exit_handler);
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
