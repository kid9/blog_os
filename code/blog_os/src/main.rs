#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
     println!("{}", _info);
     loop{}
}

static HELLO: &[u8] = b"Hello world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
     // vga_buffer::print_something();

     // use core::fmt::Write;
     // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
     // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

     println!("Hello world {}\nthis is new line \nthis is new line again", "!");
     // not exit
     // panic!("some panic message");
     loop{}
}
