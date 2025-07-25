// src/main.rs
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[panic_handler] //this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




static HELLO: &[u8] = b"Hello, world!\n";
#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    } 
    loop {}
}

