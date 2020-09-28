#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal::{pac, prelude::*, scg::{Clocks, Source}, serial::{Serial, Config}};

#[riscv_rt::entry]
fn main() -> ! {
    let cp = pac::Peripherals::take().unwrap();
    let mut pcc0 = cp.PCC0.constrain();
    let clocks = Clocks {}; // todo!
    let portb = cp.PORTB.split(&mut pcc0.portb).unwrap();
    let ptb26 = portb.ptb26.into_af3();
    let ptb25 = portb.ptb25.into_af3();
    let mut serial = Serial::lpuart0(
        cp.LPUART0,
        (ptb26, ptb25),
        Config::default(),
        clocks,
        Source::SysOsc,
        &mut pcc0.lpuart0
    ).unwrap();
    loop {
        nb::block!(serial.try_write(b'R')).ok();
    }
}
