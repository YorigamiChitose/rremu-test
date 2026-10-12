#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rremu_am::halt(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let a: u32 = 5;
    let b: u32 = 5;
    let c: u32 = 3;

    // Branch via if-else (compiler generates BEQ/BNE)
    if a == b {
        // ok
    } else {
        rremu_am::halt(1);
    }

    if a != c {
        // ok
    } else {
        rremu_am::halt(1);
    }

    // Signed comparison (BLT/BGE)
    let x: i32 = -1;
    let y: i32 = 0;
    if x < y {
        // ok
    } else {
        rremu_am::halt(1);
    }

    if x >= y {
        rremu_am::halt(1);
    }

    // Unsigned comparison (BLTU/BGEU)
    let u: u32 = 0xFFFF_FFFF;
    let v: u32 = 0;
    if u > v {
        // ok
    } else {
        rremu_am::halt(1);
    }

    // JAL/JALR via function call
    let result = double(21);
    if result != 42 { rremu_am::halt(1); }

    rremu_am::halt(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn double(x: u32) -> u32 {
    x * 2
}
