#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

use rp2040_boot2;

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    const RESET_CLR: *mut u32 = 0x4000_F000 as *mut u32;
    unsafe {
        write_volatile(RESET_CLR, 0x0000_0020); // Clear the reset bit
    }

    const RESET_RW: *mut u32 = 0x4000_C008 as *mut u32;
    unsafe {
        loop {
            let rst = read_volatile(RESET_RW); // Read the reset bit
            if rst & 0x0000_0020 != 0 {
                break; // Exit the loop if the reset bit is cleared
            }
        }
    }

    const IO_BANK0_BASE: u32 = 0x40014000;
    const GPIO25_CTRL: *mut u32 = (IO_BANK0_BASE + 0x0000_00CC) as *mut u32; // GPIO 25 Control Register
    unsafe {
        *GPIO25_CTRL = 0x05;
    }

    const GPIO_OE_SET: *mut u32 = 0xd0000024 as *mut u32; // GPIO Output Enable Register
    const GPIO_OUT_SET: *mut u32 = 0xd0000014 as *mut u32; // GPIO Output Set Register
    const GPIO_OUT_XOR: *mut u32 = 0xd000001C as *mut u32; // GPIO Output XOR Register

    unsafe {
        // Set GPIO 25 as output
        write_volatile(GPIO_OE_SET, 1 << 25); // Set GPIO 25 as output
        write_volatile(GPIO_OUT_SET, 1 << 25);
    }

    loop {
        // Toggle GPIO 25
        unsafe {
            write_volatile(GPIO_OUT_XOR, 1 << 25); // Toggle the GPIO state
        }
        for _ in 0..100_000 {
            nop();
        }
    }
}
