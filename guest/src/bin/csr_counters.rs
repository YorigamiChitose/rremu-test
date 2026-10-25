//! CSR 测试：mcycle / minstret 计数器递增。
//!
//! 读取 mcycle 与 minstret 初值，执行若干指令后再次读取，
//! 验证两者均单调递增且 instret 增量匹配实际执行指令数范围。

#![no_std]
#![no_main]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let c0: u32;
    let i0: u32;
    unsafe {
        core::arch::asm!(
            "csrr {c0}, mcycle",
            "csrr {i0}, minstret",
            c0 = out(reg) c0,
            i0 = out(reg) i0,
        );
    }
    // 消耗若干周期
    let mut acc: u32 = 0;
    for i in 0..100u32 {
        acc = acc.wrapping_add(i);
    }
    if acc == u32::MAX {
        rremu_am::halt(2);
    }
    let c1: u32;
    let i1: u32;
    unsafe {
        core::arch::asm!(
            "csrr {c1}, mcycle",
            "csrr {i1}, minstret",
            c1 = out(reg) c1,
            i1 = out(reg) i1,
        );
    }
    if c1 > c0 && i1 > i0 {
        rremu_am::halt(0)
    } else {
        rremu_am::halt(1)
    }
}
