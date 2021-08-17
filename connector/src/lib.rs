#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn sum(a: i32, b: i32) -> i32 {
    let n = a + b;
    unsafe {
        daje(n);
    }
    n
}

static mut CURRENT: i32 = 0;

#[no_mangle]
pub fn counter() -> i32 {
    let res: i32;
    unsafe {
        CURRENT += 1;
        res = CURRENT
    }
    res
}

extern {
    fn daje(n: i32);
}
