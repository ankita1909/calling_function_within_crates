#![no_std]
#![no_main]

// The libc crate allows importing functions from C.
extern crate libc;

// A list of C functions that are being imported
extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
// The main function, with its input arguments ignored, and an exit status is returned

pub extern "C" fn c_rust_ns(count: u8) -> u8 {
    unsafe {
        printf(b"Hello, World!\n" as *const u8);
    }
   // let cmd_line = std::env::args();
   // println!("No of elements in arguments is :{}",cmd_line.len());
   //print total number of values passed
   // for arg in cmd_line {
     // println!("[{}]",arg); //print all values passed 
 //}
    let input = get_input();
   // secure_input(input);
    input
}

fn get_input() -> u8{
	
	let i = 70;
	i
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

