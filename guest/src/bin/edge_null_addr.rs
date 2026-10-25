//! 边界测试：访问地址 0（未映射）触发 fault。
//!
//! AM 启动时会默认安装 mtvec，这里显式清除 mtvec 模拟「无 trap handler」场景：
//! 从地址 0 读取触发 load access fault → trap 到 mtvec=0 → 取指 0 再 fault → 自旋，
//! harness 以 Timeout 判定。

#![no_std]
#![no_main]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // 显式清除 AM 默认安装的 mtvec，恢复「无 handler」语义
    unsafe {
        core::arch::asm!("csrw mtvec, zero", options(nostack),);
    }
    // 地址 0 在 RAM 基址(0x80000000)以下，未映射 → load fault
    unsafe {
        core::arch::asm!("lw zero, 0(zero)", options(nostack),);
    }
    // 不应到达
    rremu_am::halt(0)
}
