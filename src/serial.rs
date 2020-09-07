//! Serial module.
//! 
//! This serial module is based on on-chip Low Power Universal Asynchronous Receiver/Transmitter (LPUART).
use crate::{pac, pcc};
use core::convert::Infallible;

/// Serial abstraction
pub struct Serial<LPUART, PINS> {
    lpuart: LPUART,
    pins: PINS,
}

// todo: change a name
#[derive(Clone, Copy, Debug)]
pub enum EnableError {
    Absent,
    InUse,
}

impl<PINS> Serial<pac::LPUART0, PINS> {
    pub fn lpuart0(
        lpuart0: pac::LPUART0,
        pins: PINS,
        pcc_lpuart0: &mut pcc::LPUART0,
    ) -> Result<Self, EnableError> {
        if !pcc_lpuart0.reg().read().pr().is_pr_1() {
            return Err(EnableError::Absent)
        }
        if pcc_lpuart0.reg().read().inuse().is_inuse_1() {
            return Err(EnableError::InUse)
        }
        // enable port clock
        pcc_lpuart0.reg().write(|w| w.cgc().set_bit());
        Ok(Serial { lpuart: lpuart0, pins })
    }

    pub fn release(self, pcc_lpuart: &mut pcc::LPUART0) -> (pac::LPUART0, PINS) {
        // disable port clock
        pcc_lpuart.reg().write(|w| w.cgc().clear_bit());
        // return ownership of peripherals
        (self.lpuart, self.pins)
    }
}

/// Serial pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait Pins<UART> {}

impl<PINS> embedded_hal::serial::Write<u8> for Serial<pac::LPUART0, PINS> {
    /// Write error
    type Error = Infallible;

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
    type Error = Infallible;

    /// Reads a single word from the serial interface
    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {
        todo!()
    }
}
