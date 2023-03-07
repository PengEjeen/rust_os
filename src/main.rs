#![no_std] //don't link with rust standart library
#![no_main] //don't use main fn


use core::panic::PanicInfo;

#[no_mangle]

//pub - public in mother module 
//extern "C" use rust instead of C
pub extern "C" fn _start() -> ! {
    //like main this fn is startpoint
    loop{}
}

//panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
