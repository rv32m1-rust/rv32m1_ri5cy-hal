//! Serial module
//! 
//! This serial module is based on on-chip Low Power Universal Asynchronous Receiver/Transmitter (LPUART).
use crate::{
    pac, 
    port::{ALT2, ALT3, porta::*, portb::*, portc::*/*, portd::**/},
    scg::{Clocks, Source},
    pcc::{self, EnableError},
};
use embedded_time::rate::{Extensions, Rate, Fraction, Hertz, Baud};
use core::{mem, marker::PhantomData};

/// Serial abstraction
pub struct Serial<UART, PINS> {
    uart: PhantomData<UART>,
    pins: PINS,
}

impl<PINS: Pins<pac::LPUART0>> Serial<pac::LPUART0, PINS> {
    pub fn lpuart0(
        lpuart0: pac::LPUART0,
        pins: PINS,
        config: Config,
        clocks: Clocks,
        source: Source,
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
        let source_clock = clocks.of_source(source);
        let (osr, sbr, _baud_diff) = calculate_osr_sbr_from_baudrate(
            source_clock, config.baudrate);
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
        let (parity_enable, parity_type) = match config.parity {
            Parity::ParityNone => (false, false),
            Parity::ParityEven => (true, false),
            Parity::ParityOdd => (true, true),
        };
        let mode_bit = config.parity != Parity::ParityNone; // true -> 1 parity bit -> 9 bit word
        lpuart0.ctrl.write(|w| w
            .te().set_bit()
            .re().set_bit()
            .pe().bit(parity_enable)
            .pt().bit(parity_type)
            .m().bit(mode_bit)
        );
        // 5. finished, return ownership
        Ok(Serial { uart: PhantomData, pins })
    }

    pub fn release(self, pcc_lpuart: &mut pcc::LPUART0) -> (pac::LPUART0, PINS) {
        // note(unsafe): owned type
        let lpuart = unsafe { mem::transmute::<_, pac::LPUART0>(()) };
        // close the peripheral
        lpuart.ctrl.write(|w| w
            .te().clear_bit()
            .re().clear_bit()
        );
        // disable the clock
        pcc_lpuart.disable();
        // return ownership of peripherals
        (lpuart, self.pins)
    }

    pub fn split(self) -> (Transmit<pac::LPUART0, PINS>, Receive<pac::LPUART0, PINS>) {
        (Transmit { uart: PhantomData, pins: PhantomData }, Receive { uart: PhantomData, pins: PhantomData })
    }

    // pub fn merge(tx: Transmit<pac::LPUART0, PINS>, rx: Receive<pac::LPUART0, PINS>) -> Self {
    //     Serial { uart: PhantomData, pins: tx.pins }
    // }
}

/// Transmit half of serial port
pub struct Transmit<UART, PINS> {
    uart: PhantomData<UART>,
    pins: PhantomData<PINS>,
}

impl<UART, PINS> Drop for Transmit<UART, PINS> {
    fn drop(&mut self) {
        let lpuart = unsafe { mem::transmute::<_, pac::LPUART0>(()) };
        lpuart.ctrl.write(|w| w
            .te().clear_bit()
        );
        let pcc_lpuart = unsafe { mem::transmute::<_, pcc::LPUART0>(()) };
        if lpuart.ctrl.read().re().bit_is_clear() {
            pcc_lpuart.disable();
        }
    }
}

/// Receive half of serial port
pub struct Receive<UART, PINS> {
    uart: PhantomData<UART>,
    pins: PhantomData<PINS>,
}

impl<UART, PINS> Drop for Receive<UART, PINS> {
    fn drop(&mut self) {
        let lpuart = unsafe { mem::transmute::<_, pac::LPUART0>(()) };
        lpuart.ctrl.write(|w| w
            .re().clear_bit()
        );
        let pcc_lpuart = unsafe { mem::transmute::<_, pcc::LPUART0>(()) };
        if lpuart.ctrl.read().te().bit_is_clear() {
            pcc_lpuart.disable();
        }
    }
}

const ONE: Fraction = Fraction::new(1, 1);

// OSR in [4, 32], SBR in [1, 8191]. Baud = Clock / ((OSR + 1) * SBR)
fn calculate_osr_sbr_from_baudrate(source_clock: Hertz, target_baud: Baud) -> (u8, u16, Baud) {
    let source_clock_hz = source_clock.to_generic::<u32>(ONE)
        .expect("convert source clock to hertz");
    let source_clock_hz = *source_clock_hz.integer();
    let target_baud_bps = target_baud.to_generic::<u32>(ONE)
        .expect("convert target baudrate to bps");
    let target_baud_bps = *target_baud_bps.integer();
    let mut baud_diff_bps = target_baud_bps;
    let (mut osr, mut sbr): (u32, u32) = (0, 0);
    for osr_tmp in 4..=32 {
        let mut sbr_tmp: u32 = source_clock_hz / target_baud_bps * osr_tmp;
        if sbr_tmp == 0 {
            sbr_tmp = 1;
        }
        let calc_baud_bps = source_clock_hz / (osr_tmp * sbr_tmp);
        let mut tmp_diff_bps = calc_baud_bps - target_baud_bps;
        if tmp_diff_bps > target_baud_bps - (source_clock_hz / (osr_tmp * (sbr_tmp + 1))) {
            tmp_diff_bps = target_baud_bps - (source_clock_hz / (osr_tmp * (sbr_tmp + 1)));
            sbr_tmp += 1;
        }
        if tmp_diff_bps < baud_diff_bps {
            baud_diff_bps = tmp_diff_bps;
            osr = osr_tmp;
            sbr = sbr_tmp;
        }
    }
    assert!(osr >= 4 && osr <= 32);
    assert!(sbr >= 1 && sbr <= 8191);
    (osr as u8, sbr as u16, Baud::new(baud_diff_bps))
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

/// Parity check configuration
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Parity {
    ParityNone,
    ParityEven,
    ParityOdd,
}

/// Stop bit configuration
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StopBits {
    /// 1 stop bit
    STOP1,
    /// 2 stop bits
    STOP2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Order of the bits that are transmitted and received on the wire.
pub enum Order {
    /// LSB (bit 0) is the first bit that is transmitted following that start bit
    LsbFirst,
    /// MSB (bit 9, 8, 7 or 6) is the first bit that is transmitted following that start bit
    MsbFirst,
}

/// Serial config
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Config {
    pub baudrate: Baud,
    pub parity: Parity,
    pub stopbits: StopBits,
    pub order: Order,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            baudrate: 115_200_u32.Bd(),
            parity: Parity::ParityNone,
            stopbits: StopBits::STOP1,
            order: Order::LsbFirst,
        }
    }
}

/// Serial transmit pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}
/// Serial receive pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}
/// Serial rts pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RtsPin<UART> {}
/// Serial cts pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait CtsPin<UART> {}

unsafe impl TxPin<pac::LPUART0> for PTA3<ALT2> {}
unsafe impl RxPin<pac::LPUART0> for PTA2<ALT2> {}
unsafe impl RtsPin<pac::LPUART0> for PTA4<ALT2> {}
unsafe impl CtsPin<pac::LPUART0> for PTA1<ALT2> {}

unsafe impl TxPin<pac::LPUART0> for PTB26<ALT3> {}
unsafe impl RxPin<pac::LPUART0> for PTB25<ALT3> {}
unsafe impl RtsPin<pac::LPUART0> for PTB24<ALT3> {}
unsafe impl CtsPin<pac::LPUART0> for PTB22<ALT3> {}

unsafe impl TxPin<pac::LPUART0> for PTC8<ALT3> {}
unsafe impl RxPin<pac::LPUART0> for PTC7<ALT3> {}
unsafe impl RtsPin<pac::LPUART0> for PTC10<ALT3> {}
unsafe impl CtsPin<pac::LPUART0> for PTC9<ALT3> {}

/// Serial pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait Pins<UART> {}

unsafe impl<UART, TX, RX> Pins<UART> for (TX, RX) 
where 
    TX: TxPin<UART>, 
    RX: RxPin<UART>
{}

unsafe impl<UART, TX, RX, RTS, CTS> Pins<UART> for (TX, RX, RTS, CTS) 
where 
    TX: TxPin<UART>, 
    RX: RxPin<UART>, 
    RTS: RxPin<UART>, 
    CTS: RxPin<UART>
{}

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
