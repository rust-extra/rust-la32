#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::fmt::Write;
use uart_16550::MmioSerialPort;

const UART_BASE_ADDRESS: usize = 0x1fe0_01e0;

global_asm!(
    "
        .text
        .align  2
        .global _start
        .type   _start, @function
    _start:
        la      $sp, _sp0
        b       main

        .data
        .align	4
        .space	0x10000
    _sp0:
    "
);

fn die() -> ! {
    loop {
        unsafe {
            asm!("idle 0");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    die();
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut uart = unsafe { MmioSerialPort::new(UART_BASE_ADDRESS) };

    uart.init();
    let _ = uart.write_str("Hello Rust LA32!\n");

    die();
}
