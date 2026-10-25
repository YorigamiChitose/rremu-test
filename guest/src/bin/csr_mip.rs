//! CSR 测试：mip 反映设备 pending 中断。
//!
//! 置 CLINT msip=1 后读 mip，验证 MSIP 位（bit3）可见。
//! 设备位由 rremu 每步投影到 mip（软件只读）。

#![no_std]
#![no_main]

use rremu_am::{clint_set_msip, halt};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    clint_set_msip(1);
    let mip: u32;
    unsafe {
        core::arch::asm!("csrr {mip}, mip", mip = out(reg) mip);
    }
    clint_set_msip(0);

    if mip & 0x8 != 0 { halt(0) } else { halt(1) }
}
