#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    let sbss_ptr = sbss as *const () as usize;
    let ebss_ptr = ebss as *const () as usize;
    (sbss_ptr..ebss_ptr).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    loop {}
}
