#![no_std]
#![no_main]

use core::ptr::{addr_of_mut, read_volatile, write_volatile};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(99)
}

static mut BUF: [u32; 8] = [0; 8];

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        // Word store/load
        write_volatile(addr_of_mut!(BUF[0]), 0xDEADBEEF);
        let w = read_volatile(addr_of_mut!(BUF[0]));
        if w != 0xDEADBEEF { rremu_am::halt(10); }

        // Byte store/load
        let p = addr_of_mut!(BUF[0]) as *mut u8;
        write_volatile(p, 0xAB);
        write_volatile(p.add(1), 0xCD);
        let b0 = read_volatile(p);
        let b1 = read_volatile(p.add(1));
        if b0 != 0xAB { rremu_am::halt(20); }
        if b1 != 0xCD { rremu_am::halt(21); }

        // Halfword store/load
        let h = addr_of_mut!(BUF[0]) as *mut u16;
        write_volatile(h.add(2), 0xBEEF);
        let hv = read_volatile(h.add(2));
        if hv != 0xBEEF { rremu_am::halt(30); }
    }

    rremu_am::halt(0)
}
