#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use blog_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
     println!("{}", info);
     loop{}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
     blog_os::test_panic_handler(info);
}

// static HELLO: &[u8] = b"Hello world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
     // vga_buffer::print_something();

     // use core::fmt::Write;
     // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
     // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

     println!("Hello world {}\nthis is new line \nthis is new line again", "!");
     // not exit
     // panic!("some panic message");
     #[cfg(test)]
     test_main();

     loop{}
}
