//! 设备测试：CLINT 定时器计数。
//!
//! 读取 mtime（t0），执行若干无副作用指令后再读 mtime（t1）。
//! 若 t1 > t0（计数器单调递增），halt(0)；否则 halt(1)。
//! 此用例只验证 CLINT 的时序计数行为，中断触发由 CTE 测试覆盖。

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let t0 = rremu_am::clint_get_time();

    // 消耗若干周期：空转循环（编译器不应优化掉，因 t0 后续被使用）
    let mut acc: u32 = 0;
    for i in 0..100u32 {
        acc = acc.wrapping_add(i);
    }
    // 防止 acc 被优化掉
    if acc == u32::MAX {
        rremu_am::halt(2);
    }

    let t1 = rremu_am::clint_get_time();

    if t1 > t0 {
        rremu_am::halt(0)
    } else {
        rremu_am::halt(1)
    }
}
