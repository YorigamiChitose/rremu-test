//! ISA 测试：A 扩展 lr.w / sc.w（成功与失败路径）。
//!
//! lr.w 建立保留，sc.w 成功写回并返回 0；普通 store 使保留失效后
//! sc.w 失败返回非 0 且不写内存。

#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, read_volatile, write_volatile};

static mut CELL: u32 = 0;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        let p = addr_of_mut!(CELL);
        write_volatile(p, 0x123);

        // lr.w 读旧值并建立保留
        let loaded: u32;
        core::arch::asm!(
            "lr.w {loaded}, ({a})",
            a = in(reg) p as u32,
            loaded = out(reg) loaded,
        );
        if loaded != 0x123 {
            rremu_am::halt(1);
        }

        // sc.w 成功：写 0x456，返回 0
        let ok: u32;
        core::arch::asm!(
            "sc.w {ok}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0x456,
            ok = out(reg) ok,
        );
        if ok != 0 {
            rremu_am::halt(1);
        }
        if read_volatile(p) != 0x456 {
            rremu_am::halt(1);
        }

        // 再次 lr.w 建立保留，但普通 store 使其失效
        core::arch::asm!(
            "lr.w {t}, ({a})",
            a = in(reg) p as u32,
            t = out(reg) _,
        );
        write_volatile(p, 0x789); // 普通 store 清除保留

        // sc.w 失败：返回非 0，且不写内存
        let ok: u32;
        core::arch::asm!(
            "sc.w {ok}, {v}, ({a})",
            a = in(reg) p as u32,
            v = in(reg) 0x999,
            ok = out(reg) ok,
        );
        if ok == 0 {
            rremu_am::halt(1);
        } // 应失败
        if read_volatile(p) != 0x789 {
            rremu_am::halt(1);
        } // sc.w 未写
    }

    rremu_am::halt(0)
}
