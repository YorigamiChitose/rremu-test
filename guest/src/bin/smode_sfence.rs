//! S-mode 测试：`sfence.vma` 作为 NOP 执行后继续运行。

#![no_std]
#![no_main]

use rremu_am::{enter_supervisor, halt, trap_init};
use rremu_guest::supervisor_exit_handler;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

extern "C" fn s_entry() -> ! {
    unsafe {
        core::arch::asm!("sfence.vma", "sfence.vma zero, zero");
    }
    halt(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(supervisor_exit_handler);
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
