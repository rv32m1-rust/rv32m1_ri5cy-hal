#![feature(asm)]
#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{pac, prelude::*};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let mut pcc0 = cp.PCC0.constrain();
    let porta = cp.PORTA.split(&mut pcc0.porta).unwrap();
    let mut gpioa = cp.GPIOA.constrain();
    // The red led light is connected to pta24 on vega board.
    let mut pa24 = porta.pta24.unlocked().into_push_pull_output(&mut gpioa);
    // Do `pa24.set_high()` would set the light on, `set_low` for off.
    pa24.set_high().unwrap();
    loop {
        pa24.toggle().unwrap();
        delay();
    }
}

fn delay() {
    for _ in 0..800_000 {
        unsafe { asm!("nop") };
    }
}
