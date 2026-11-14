#![no_std]
#![no_main]

mod console;

use console::sys_exit;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! print {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: expr $(, $($arg: tt)+)?) => {
        $crate::print!(concat!($fmt, "\n") $(, $($arg)+)?);
    }
}

#[unsafe(no_mangle)]
extern "C" fn _start() {
    println!("Hello, world!");
    sys_exit(0);
}
