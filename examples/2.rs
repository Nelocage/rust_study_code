//#![no_std]这个好像不可以放在use语句的下面




//!的用法：注意这个是#[panic_handler],而正常是#！[no_std],中间缺个！
//The function should never return, so it is marked as a diverging function by returning the “never” type !.


#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO:&[u8]=b"hello,world";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    let vga_buffer=0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;

        }
    }
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




