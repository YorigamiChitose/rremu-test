//! rremu-test guest 公共测试辅助。
//!
//! S-mode 代码无法直接执行 `halt`（退出协议只接受 M-mode `ecall`）。
//! `supervisor_exit_handler` 作为 M-mode 陷阱处理函数，把 S-mode
//! `halt(code)` 触发的 `ECALL_FROM_S` 桥接回 M-mode 并以同一 code 退出。

#![no_std]

use rremu_am::{Context, Event, halt};

static mut S_EXIT_CODE: i32 = 0;

extern "C" fn m_halt() -> ! {
    halt(unsafe { core::ptr::read_volatile(core::ptr::addr_of!(S_EXIT_CODE)) })
}

/// 安装到 `trap_init` 的 M-mode handler：只接受 S-mode ecall，其余按默认错误退出。
pub extern "C" fn supervisor_exit_handler(_ev: Event, ctx: *mut Context) -> *mut Context {
    unsafe {
        if (*ctx).mcause == 9 {
            S_EXIT_CODE = (*ctx).gpr[10] as i32;
            // 返回 M-mode：MPP=M，mepc 指向 m_halt。
            (*ctx).mstatus = ((*ctx).mstatus & !(0b11 << 11)) | (0b11 << 11);
            (*ctx).mepc = m_halt as *const () as usize as u32;
            return ctx;
        }
        halt(0x10)
    }
}
