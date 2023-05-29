#![no_std]
#![no_main]

use common::sprintln;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    common::init_uart();

    sprintln!("Hello, world!");

    loop {}
}
