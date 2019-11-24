//! Serial module.
//! 
//! This serial module is based on on-chip Low Power Universal Asynchronous Receiver/Transmitter (LPUART).
use crate::{pac, pcc};

/// Serial abstraction
pub struct Serial<LPUART, PINS> {
    _lpuart: LPUART,
    _pins: PINS,
}

// todo: change a name
#[derive(Clone, Copy, Debug)]
pub enum OpenError {
    Absent,
    InUse,
}

impl<TX, RX> Serial<pac::LPUART0, (TX, RX)> {
    pub fn lpuart0(
        lpuart0: pac::LPUART0,
        pins: (TX, RX),
        pcc_lpuart0: &mut pcc::LPUART0,
    ) -> Result<Self, OpenError> {
        if !pcc_lpuart0.reg().read().pr().is_pr_1() {
            return Err(OpenError::Absent)
        }
        if pcc_lpuart0.reg().read().inuse().is_inuse_1() {
            return Err(OpenError::InUse)
        }
        // todo
        Ok(Serial { _lpuart: lpuart0, _pins: pins })
    }
}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<LPUART> {}

/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<LPUART> {}

