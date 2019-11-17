#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{self as hal, prelude::*, pac};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let mut pcc0 = cp.PCC0.constrain();
    let gpioa = (cp.GPIOA, cp.PORTA).split(&mut pcc0.porta).unwrap();
    let mut pta24 = gpioa.pta24.into_push_pull_output();
    // do something with pta24
    pta24.set_high().unwrap();
    // todo: delay here
    // pta24.set_low().unwrap();
    // free gpioa parts
    let (_gpioa, _porta) = hal::gpio::gpioa::Parts { 
        pta24: pta24.into_floating_input(),
    ..gpioa }.free(&mut pcc0.porta);
    loop {}
}
