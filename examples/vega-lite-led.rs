#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{prelude::*, pac};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let mut pcc0 = cp.PCC0.constrain();
    let gpioa = (cp.GPIOA, cp.PORTA).split(&mut pcc0.porta).unwrap();
    unsafe { &(*pac::PORTA::ptr()).pcr24.write(|w| w.mux().mux_1()) } ;
    let mut pta24 = gpioa.pta24.into_open_drain_output();
    loop {
        pta24.set_low().unwrap();
        pta24.set_high().unwrap();
    }
}
