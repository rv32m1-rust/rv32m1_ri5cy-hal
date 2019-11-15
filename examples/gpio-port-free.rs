#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{self as hal, prelude::*, pac};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let gpioa = (cp.GPIOA, cp.PORTA).split().unwrap();
    let mut pta24 = gpioa.pta24.into_push_pull_output();
    pta24.set_low().unwrap();
    let (_GPIOA, _PORTA) = hal::gpio::gpioa::Parts { 
        pta24: pta24.into_floating_input(),
    ..gpioa }.free();
    loop {}
}
