//! syscon 测试：写 reboot 值应以不同于 poweroff 的退出码终止。

#![no_std]
#![no_main]

use rremu_am::halt;

const SYSCON_BASE: u32 = 0x0010_0000;
const REBOOT: u32 = 0x7777;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe { core::ptr::write_volatile(SYSCON_BASE as *mut u32, REBOOT) };
    // 若 syscon 未触发宿主停机，则超时失败。
    loop {
        core::hint::spin_loop();
    }
}
