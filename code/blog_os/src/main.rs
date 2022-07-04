#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
     loop{}
}

static HELLO: &[u8] = b"Hello world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
     let vga_buffer = 0xb8000 as *mut u8;
     for (i, &byte) in HELLO.iter().enumerate() {
          // we manually set address 0xb8000 as pointer, it may be a invalid address
          // so unsafe is used to surround the pointer operation
          unsafe {
               // set vga ascii byte
               *vga_buffer.offset(i as isize * 2) = byte;
               // set vga color byte (light cyan)
               *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
          }
     }
     // not exit
     loop{}
}
