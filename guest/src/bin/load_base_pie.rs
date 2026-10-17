//! 加载基址测试：ELF 段与入口整体平移到非链接地址后仍能执行。
//!
//! guest 链接在 0x80000000，harness 以 load_base=0x1000 加载；
//! 若入口或段未同步偏移，将无法执行到此处的 halt(0)。

#![no_std]
#![no_main]

use rremu_am::halt;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    halt(0)
}
