#![no_std]
#![no_main]

mod dtb;
mod serial;
mod drivers {
    pub mod pl011;
}

use core::arch::asm;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
