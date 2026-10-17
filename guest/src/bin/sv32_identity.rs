//! Sv32 端到端测试：S-mode 在 satp 开启后通过页表执行。
//!
//! 在 M-mode 建立 4 个 4MiB 大页，覆盖代码/数据/栈所在的 16MiB RAM，
//! 然后进入 S-mode；若页表或翻译有误，取指/退出路径会失败。

#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, write_volatile};

use rremu_am::{enter_supervisor, halt, trap_init};
use rremu_guest::supervisor_exit_handler;

const PTE_V: u32 = 1 << 0;
const PTE_R: u32 = 1 << 1;
const PTE_W: u32 = 1 << 2;
const PTE_X: u32 = 1 << 3;
const SATP_SV32: u32 = 1 << 31;
const RAM_BASE: u32 = 0x8000_0000;
const SUPERPAGE: u32 = 0x40_0000;
const RAM_SIZE: u32 = 16 * 1024 * 1024;

#[repr(C, align(4096))]
struct PageTable([u32; 1024]);

static mut ROOT: PageTable = PageTable([0; 1024]);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

fn map_identity_superpages() -> u32 {
    let root = unsafe { addr_of_mut!(ROOT.0) as *mut u32 };
    let mut pa = RAM_BASE;
    while pa < RAM_BASE + RAM_SIZE {
        let ppn = (pa >> 12) & 0x3f_ffff;
        let pte = PTE_V | PTE_R | PTE_W | PTE_X | (ppn << 10);
        let idx = (pa >> 22) as usize;
        unsafe { write_volatile(root.add(idx), pte) };
        pa += SUPERPAGE;
    }
    root as u32
}

extern "C" fn s_entry() -> ! {
    halt(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(supervisor_exit_handler);
    let root = map_identity_superpages();
    unsafe {
        core::arch::asm!(
            "csrw satp, {satp}",
            satp = in(reg) SATP_SV32 | (root >> 12),
            options(nostack),
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, 0, 0)
}
