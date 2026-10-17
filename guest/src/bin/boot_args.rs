//! 启动注入测试：guest 读取 AM 捕获的 a0(hartid) / a1(dtb)。

#![no_std]
#![no_main]

use rremu_am::{boot_dtb, boot_hartid, halt};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    if boot_hartid() == 7 && boot_dtb() == 0x8000_1000 {
        halt(0)
    } else {
        halt(1)
    }
}
