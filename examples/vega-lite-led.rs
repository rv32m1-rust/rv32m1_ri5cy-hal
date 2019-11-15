#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{prelude::*, pac};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let gpioa = (cp.GPIOA, cp.PORTA).split();
    let mut pta24 = gpioa.pta24.into_push_pull_output();
    pta24.set_low().unwrap();
    loop {}
}
