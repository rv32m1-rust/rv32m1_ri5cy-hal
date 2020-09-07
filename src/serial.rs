//! Serial module.
//! 
//! This serial module is based on on-chip Low Power Universal Asynchronous Receiver/Transmitter (LPUART).
use crate::{pac, pcc::{self, EnableError}};

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

impl<PINS> Serial<pac::LPUART0, PINS> {
    pub fn lpuart0(
        lpuart0: pac::LPUART0,
        pins: PINS,
        pcc_lpuart0: &mut pcc::LPUART0,
    ) -> Result<Self, EnableError> {
        pcc_lpuart0.try_enable()?;
        Ok(Serial { lpuart: lpuart0, pins })
    }

    pub fn release(self, pcc_lpuart: &mut pcc::LPUART0) -> (pac::LPUART0, PINS) {
        // disable the peripheral
        pcc_lpuart.disable();
        // return ownership of peripherals
        (self.lpuart, self.pins)
    }
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
