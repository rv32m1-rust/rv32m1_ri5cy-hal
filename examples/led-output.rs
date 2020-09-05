#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{pac, prelude::*};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let mut pcc0 = cp.PCC0.constrain();
    let gpioa = (cp.GPIOA, cp.PORTA).split(&mut pcc0.porta).unwrap();
    // The red led light is connected to pta24 on vega board.
    let mut pta24 = gpioa.pta24.into_push_pull_output();
    let mut pta23 = gpioa.pta23.into_push_pull_output();
    // Do `pta24.set_high()` would set the light on, `set_low` for off.
    pta24.try_set_high().unwrap();
    pta23.try_set_high().unwrap();
    loop {
        pta24.try_toggle().unwrap();
        delay();
        pta23.try_toggle().unwrap();
        delay();
    }
}

fn delay() {
    for _ in 0..800_000 {
        // nop
    }
}
