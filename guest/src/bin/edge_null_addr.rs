//! 边界测试：访问地址 0（未映射）触发 fault。
//!
//! 从地址 0 读取触发 load access fault，模拟器以 faulted 终止。

#![no_std]
#![no_main]
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { rremu_am::halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // 地址 0 在 RAM 基址(0x80000000)以下，未映射 → load fault
    unsafe {
        core::arch::asm!(
            "lw zero, 0(zero)",
            options(nostack),
        );
    }
    // 不应到达
    rremu_am::halt(0)
}
