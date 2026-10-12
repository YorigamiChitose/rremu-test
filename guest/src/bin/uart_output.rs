//! 设备测试：UART 输出。
//!
//! 经由 rremu-am::putc 依次输出 "OK" 两字节后 halt(0)。
//! Harness 通过 uart_expect = "OK" 校验输出缓冲。

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    rremu_am::putc(b'O');
    rremu_am::putc(b'K');
    rremu_am::halt(0)
}
