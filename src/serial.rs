//! Serial module.
//! 
//! This serial module is based on on-chip Low Power Universal Asynchronous Receiver/Transmitter (LPUART).
use crate::{pac, pcc::{self, EnableError}};
use embedded_time::rate::{Hertz, Baud};

pub struct Clocks {
    freq: Hertz,
} // todo

/// Serial abstraction
pub struct Serial<LPUART, PINS> {
    lpuart: LPUART,
    pins: PINS,
}

/// Serial error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
}

pub enum Parity {
    ParityNone,
    ParityEven,
    ParityOdd,
}

pub enum StopBits {
    /// 1 stop bit
    STOP1,
    /// 2 stop bits
    STOP2,
}

/// Order of the bits that are transmitted and received on the wire.
pub enum Order {
    /// LSB (bit 0) is the first bit that is transmitted following that start bit
    LsbFirst,
    /// MSB (bit 9, 8, 7 or 6) is the first bit that is transmitted following that start bit
    MsbFirst,
}

pub struct Config {
    pub baudrate: Baud,
    pub parity: Parity,
    pub stopbits: StopBits,
    pub order: Order,
}

impl<PINS> Serial<pac::LPUART0, PINS> {
    pub fn lpuart0(
        lpuart0: pac::LPUART0,
        pins: PINS,
        config: Config,
        clocks: Clocks,
        pcc_lpuart0: &mut pcc::LPUART0,
    ) -> Result<Self, EnableError> {
        // 1. peripheral power on
        // enable peripheral clock
        pcc_lpuart0.try_enable()?;
        // reset device
        lpuart0.global.write(|w| w.rst().set_bit());
        lpuart0.global.write(|w| w.rst().clear_bit());
        // 2. set BAUD baudrate regitser value
        // calculate best config from baudrate settings
        let (osr, sbr, baud_diff) = calculate_osr_sbr_from_baudrate(
            clocks.freq, config.baudrate);
        let both_edge = osr >= 4 && osr <= 7;
        let stop_bits = match config.stopbits {
            StopBits::STOP1 => false,
            StopBits::STOP2 => true,
        };
        // note(unsafe): value is valid from function
        lpuart0.baud.write(|w| unsafe { w
            .osr().bits(osr - 1) // set osr bits
            .sbr().bits(sbr) // set sbr bits
            .bothedge().bit(both_edge)
            .sbns().bit(stop_bits)
            .m10().clear_bit() // disable word bit 10 by now
        });
        // 3. set STAT status register
        let msbf = match config.order {
            Order::LsbFirst => false,
            Order::MsbFirst => true,
        };
        lpuart0.stat.write(|w| w.msbf().bit(msbf));
        // 4. set CTRL control register
        let mode_bit = stop_bits; // true -> 2 bits -> 9 bit word
        let (parity_enable, parity_type) = match config.parity {
            Parity::ParityNone => (false, false),
            Parity::ParityEven => (true, false),
            Parity::ParityOdd => (true, true),
        };
        lpuart0.ctrl.write(|w| w
            .te().set_bit()
            .re().set_bit()
            .pe().bit(parity_enable)
            .pt().bit(parity_type)
            .m().bit(mode_bit)
        );
        // 5. finished, return ownership
        Ok(Serial { lpuart: lpuart0, pins })
    }

    pub fn release(self, pcc_lpuart: &mut pcc::LPUART0) -> (pac::LPUART0, PINS) {
        // disable the peripheral
        pcc_lpuart.disable();
        // return ownership of peripherals
        (self.lpuart, self.pins)
    }
}

// OSR in [4, 32], SBR in [1, 8191]. Baud = Clock / ((OSR + 1) * SBR)
fn calculate_osr_sbr_from_baudrate(source_clock: Hertz, target_baud: Baud) -> (u8, u16, Baud) {
    let mut baud_diff = target_baud;
    let (mut osr, mut sbr): (usize, usize) = (0, 0);
    for osr_tmp in 4..=32 {
        // let mut sbr_tmp = source_clock / (target_baud * osr_tmp);
        // if sbr_tmp == 0 {
        //     sbr_tmp = 1;
        // }
        // let calc_baud = source_clock / (osr_tmp * sbr_tmp);
        // let mut tmp_diff = calc_baud - target_baud;
        // if tmp_diff > target_baud - (source_clock / (osr_tmp * (sbr_tmp + 1))) {
        //     tmp_diff = target_baud - (source_clock / (osr_tmp * (sbr_tmp + 1)));
        //     sbr_tmp += 1;
        // }
        // if tmp_diff < baud_diff {
        //     baud_diff = tmp_diff;
        //     osr = osr_tmp;
        //     sbr = sbr_tmp;
        // }
        todo!()
    }
    assert!(osr >= 4 && osr <= 32);
    assert!(sbr >= 1 && sbr <= 8191);
    (osr as u8, sbr as u16, baud_diff)
}

/// Serial pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait Pins<UART> {}

impl<PINS> embedded_hal::serial::Write<u8> for Serial<pac::LPUART0, PINS> {
    /// Write error
    type Error = Error;

    /// Writes a single word to the serial interface
    fn try_write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        todo!("{:?}", word)
    }

    /// Ensures that none of the previously written words are still buffered
    fn try_flush(&mut self) -> nb::Result<(), Self::Error> {
        todo!()
    }
}

impl<PINS> embedded_hal::serial::Read<u8> for Serial<pac::LPUART0, PINS> {
    /// Read error
    type Error = Error;

    /// Reads a single word from the serial interface
    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {
        todo!()
    }
}
