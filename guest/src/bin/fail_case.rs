//! 断言矩阵测试：Fail（以非零退出码终止）。
//!
//! 直接 halt(1)，harness 断言 Expect::Fail（exit code != 0）。

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    rremu_am::halt(1)
}
