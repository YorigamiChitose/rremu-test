//! sip 回归：STIP/SEIP 为只读，SSIP 才可由软件写入。

#![no_std]
#![no_main]

use rremu_am::halt;

const MIP_STIP: u32 = 1 << 5;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let sip: u32;
    unsafe {
        core::arch::asm!(
            "csrw mip, {mip}",
            "csrw sip, zero",
            "csrr {sip}, sip",
            mip = in(reg) MIP_STIP,
            sip = out(reg) sip,
            out("t0") _,
        );
    }
    if sip & MIP_STIP == MIP_STIP {
        halt(0)
    } else {
        halt(1)
    }
}
