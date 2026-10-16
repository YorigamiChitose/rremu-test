//! 正常退出基线：rust_main 直接 halt(0)。
//!
//! 验证最小 guest 的正常退出路径——`halt(0)`（ecall + a7=EXIT_MAGIC）
//! 被宿主以 a0 为退出码 0 终止，harness 断言 Expect::Pass。

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    rremu_am::halt(0)
}
