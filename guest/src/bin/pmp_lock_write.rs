//! PMP 回归：锁定 entry 的 pmpcfg/pmpaddr 写入必须被忽略。

#![no_std]
#![no_main]

use rremu_am::halt;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let cfg: u32;
    let addr: u32;
    unsafe {
        core::arch::asm!(
            // 4GiB NAPOT + R/W/X，然后锁定。
            "li t0, 0x1fffffff",
            "csrw pmpaddr0, t0",
            "li t0, 0x9f",
            "csrw pmpcfg0, t0",
            // 尝试覆盖 cfg 和 addr；两者都应被忽略。
            "li t0, 0x1f",
            "csrw pmpcfg0, t0",
            "li t0, 0x200",
            "csrw pmpaddr0, t0",
            "csrr {cfg}, pmpcfg0",
            "csrr {addr}, pmpaddr0",
            cfg = out(reg) cfg,
            addr = out(reg) addr,
            out("t0") _,
        );
    }
    if cfg & 0xff == 0x9f && addr == 0x1fff_ffff {
        halt(0)
    } else {
        halt(1)
    }
}
