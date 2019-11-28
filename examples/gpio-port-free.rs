#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{self as hal, pac, prelude::*};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let mut pcc0 = cp.PCC0.constrain();
    let porta = cp.PORTA.split(&mut pcc0.porta).unwrap();
    let mut gpioa = cp.GPIOA.constrain();
    let mut pa24 = porta.pta24.unlocked().into_push_pull_output(&mut gpioa);
    // do something with pta24
    pa24.set_high().unwrap();
    // todo: delay here
    // pa24.set_low().unwrap();
    // free gpioa parts
    let _porta = hal::port::porta::Parts {
        pta24: pa24.free().unknown(),
        ..porta
    }
    .free(&mut pcc0.porta);
    loop {}
}
