//! DTB 加载测试：harness 将 DTB 写到 RAM 顶部并设置 boot a1。
//!
//! guest 通过 AM 捕获的 a1 读取 DTB 魔数，验证内容确实被加载。

#![no_std]
#![no_main]

use rremu_am::{boot_dtb, halt};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let addr = boot_dtb();
    if addr == 0 {
        halt(1);
    }
    let p = addr as *const u8;
    let bytes = unsafe {
        [
            core::ptr::read_volatile(p),
            core::ptr::read_volatile(p.add(1)),
            core::ptr::read_volatile(p.add(2)),
            core::ptr::read_volatile(p.add(3)),
        ]
    };
    if bytes == [0xd0, 0x0d, 0xfe, 0xed] {
        halt(0)
    } else {
        halt(1)
    }
}
