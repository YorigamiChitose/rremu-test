//! FP e2e：c.flw/c.fsw 的压缩寄存器与偏移缩放。
#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, read_volatile, write_volatile};
use rremu_am::fp::{FS_CLEAN, write_fs};

static mut BUF: [u32; 4] = [0; 4];

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    write_fs(FS_CLEAN);
    let p = unsafe { addr_of_mut!(BUF[0]) };
    unsafe { write_volatile(p.add(1), 0x4040_0000) };

    // 不声明 out("f8")：soft-float ABI 下编译器不会使用 FPR；若声明 clobber，
    // 编译器会在函数入口生成 `fsw fs0, ...`，而那时 FS 仍为 Off，会先触发 illegal。
    unsafe {
        let got: u32;
        core::arch::asm!(
            "c.flw f8, 4(a0)",
            "fmv.x.w {got}, f8",
            in("a0") p,
            got = out(reg) got,
            options(nostack),
        );
        if got != 0x4040_0000 {
            rremu_am::halt(10);
        }

        core::arch::asm!(
            "fmv.w.x f8, {value}",
            "c.fsw f8, 4(a0)",
            in("a0") p,
            value = in(reg) 0x4000_0000u32,
            options(nostack),
        );
        if read_volatile(p.add(1)) != 0x4000_0000 {
            rremu_am::halt(11);
        }
    }

    rremu_am::halt(0)
}
