//! CTE 测试：上下文切换（handler 返回不同 Context）。
//!
//! guest 调用 ecall(Yield)，handler 将 mepc 设为 check_fn 地址、
//! a0 设为 0xDEAD，返回预备的 CTX_B 指针。
//! mret 后从 check_fn 继续执行：验证 a0 == 0xDEAD，并用递归 + 局部数组
//! 验证切换后的 sp 正确（栈可用）。

#![no_std]
#![no_main]
use rremu_am::{Context, Event, trap_init, halt};

// 预备的目标上下文
static mut CTX_B: Context = Context { gpr: [0; 32], mcause: 0, mstatus: 0, mepc: 0 };

/// 使用栈：递归调用 + 局部数组。若切换后的 sp 错误，ra 保存/恢复会错乱。
#[inline(never)]
fn stack_probe(depth: u32) -> u32 {
    let local: [u32; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    if depth == 0 {
        0
    } else {
        stack_probe(depth - 1).wrapping_add(local[(depth as usize) & 3])
    }
}

extern "C" fn check_fn() -> ! {
    let a0: u32;
    unsafe { core::arch::asm!("mv {}, a0", out(reg) a0); }
    if a0 != 0xDEAD { halt(1); }
    // stack_probe(3) = 0xBB + 0xCC + 0xDD = 0x264
    let r = stack_probe(3);
    if r == 0x264 { halt(0) } else { halt(1) }
}

extern "C" fn handler(ev: Event, ctx: *mut Context) -> *mut Context {
    if ev == Event::Yield {
        unsafe {
            // 从当前上下文复制 mstatus 和 sp
            let mstatus = (*ctx).mstatus;
            let sp = (*ctx).gpr[2];
            // 配置 CTX_B
            CTX_B.mepc = check_fn as *const () as usize as u32;
            CTX_B.mstatus = mstatus;
            CTX_B.gpr[2] = sp;
            CTX_B.gpr[10] = 0xDEAD; // a0
            // 推进原 ctx 的 mepc（不使用，但保持一致性）
            let mepc = (*ctx).mepc;
            let half = core::ptr::read_volatile(mepc as *const u16);
            let len = if half & 0x3 == 0x3 { 4 } else { 2 };
            (*ctx).mepc = mepc.wrapping_add(len);
            // 返回 CTX_B
            core::ptr::addr_of_mut!(CTX_B)
        }
    } else {
        ctx
    }
}

#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { halt(1) }
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    trap_init(handler);
    let magic: u32 = u32::MAX;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") magic,
            options(nostack),
        );
    }
    // 正常情况下不会到达这里（handler 切到 check_fn）
    halt(1)
}
