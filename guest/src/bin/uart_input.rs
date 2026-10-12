//! 设备测试：UART 输入。
//!
//! Harness 预装 'A' 到 UART FIFO，guest 轮询 uart_getc() 直至取回该字节。
//! 取回 'A' → halt(0)，否则 halt(1)。

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // 轮询 UART 输入直至 Data Ready
    let ch = loop {
        if let Some(b) = rremu_am::uart_getc() {
            break b;
        }
    };
    if ch == b'A' {
        rremu_am::halt(0)
    } else {
        rremu_am::halt(1)
    }
}
