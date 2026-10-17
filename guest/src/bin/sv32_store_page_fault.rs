//! Sv32 端到端测试：S-mode 向未映射 VA 存储触发 store page fault。

#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, write_volatile};

use rremu_am::{Context, Event, enter_supervisor, halt, pmp_allow_all, trap_init};

const PTE_V: u32 = 1 << 0;
const PTE_R: u32 = 1 << 1;
const PTE_W: u32 = 1 << 2;
const PTE_X: u32 = 1 << 3;
const SATP_SV32: u32 = 1 << 31;
const RAM_BASE: u32 = 0x8000_0000;
const UNMAPPED_VA: u32 = 0x4000_0000;

#[repr(C, align(4096))]
struct PageTable([u32; 1024]);

static mut ROOT: PageTable = PageTable([0; 1024]);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt(1)
}

fn map_code_superpage() -> u32 {
    let root = unsafe { addr_of_mut!(ROOT.0) as *mut u32 };
    let ppn = (RAM_BASE >> 12) & 0x3f_ffff;
    let pte = PTE_V | PTE_R | PTE_W | PTE_X | (ppn << 10);
    unsafe { write_volatile(root.add((RAM_BASE >> 22) as usize), pte) };
    root as u32
}

extern "C" fn expect_store_page_fault(_ev: Event, ctx: *mut Context) -> *mut Context {
    unsafe {
        if (*ctx).mcause == 15 {
            halt(0)
        } else {
            halt(1)
        }
    }
}

extern "C" fn s_entry() -> ! {
    unsafe {
        core::arch::asm!("sw zero, 0(a0)", options(noreturn));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    pmp_allow_all();
    trap_init(expect_store_page_fault);
    let root = map_code_superpage();
    unsafe {
        core::arch::asm!(
            "csrw satp, {satp}",
            satp = in(reg) SATP_SV32 | (root >> 12),
            options(nostack),
        );
    }
    enter_supervisor(s_entry as *const () as usize as u32, UNMAPPED_VA, 0)
}
