//! RTC + syscon 测试：读取 Goldfish RTC，写入 poweroff 触发模拟器停机。

#![no_std]
#![no_main]

use rremu_am::halt;

const RTC_BASE: u32 = 0x0010_1000;
const SYSCON_BASE: u32 = 0x0010_0000;
const POWEROFF: u32 = 0x5555;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let now = unsafe { core::ptr::read_volatile(RTC_BASE as *const u32) };
    if now == 0 {
        halt(1);
    }
    unsafe { core::ptr::write_volatile(SYSCON_BASE as *mut u32, POWEROFF) };
    // 若 syscon 未触发 Halt，则超时失败。
    loop {
        core::hint::spin_loop();
    }
}
