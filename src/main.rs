#![no_std]
#![no_main]

// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;

use libc::*;

// Entry point for this program.
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    // Since we are passing a C string the final null character is mandatory
    let hello = "Hello, minimal Rust!\n\0";
    unsafe {
        printf(hello.as_ptr() as *const _);

        let fname = "hello.txt\0".as_ptr() as *const _;
        let mode = "w\0".as_ptr() as *const _;

        let file = fopen(fname, mode);
        fprintf(file, hello.as_ptr() as *const _);
        fclose(file);
    }
    0
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
