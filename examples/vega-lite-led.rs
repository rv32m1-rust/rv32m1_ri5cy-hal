#![feature(asm)]
#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{pac, prelude::*};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let mut pcc0 = cp.PCC0.constrain();
    let gpioa = (cp.GPIOA, cp.PORTA).split(&mut pcc0.porta).unwrap();
    let mut pta22 = gpioa.pta22.into_push_pull_output();
    let mut pta23 = gpioa.pta23.into_push_pull_output();
    let mut pta24 = gpioa.pta24.into_push_pull_output();
    pta22.set_high().unwrap();
    pta23.set_high().unwrap();
    pta24.set_high().unwrap();
    loop {}
}
