//! Peripheral Clock Controller (PCC)

use crate::pac::{pcc0, PCC0};

pub trait PccExt {
    fn constrain(self) -> Pcc;
}

macro_rules! pcc_impl {
    ($($REGX: ident, $regx: ident, $PCC_REGX: ident, $pcc_regx: ident, $doc: expr;)+) => {
impl PccExt for PCC0 {
    fn constrain(self) -> Pcc {
        Pcc {
            $( $regx: $REGX { _ownership: () }, )+
        }
    }
}

pub struct Pcc {
    $(
        #[doc = $doc]
        pub $regx: $REGX,
    )+
}
$(
    #[doc = $doc]
    pub struct $REGX {
        _ownership: ()
    }

    impl $REGX {
        fn reg(&self) -> &pcc0::$PCC_REGX {
            unsafe { &(*PCC0::ptr()).$pcc_regx }
        }
        pub(crate) fn try_enable(&self) -> core::result::Result<(), EnableError> {
            // if the port is not absent on this device, throw an error
            if !self.reg().read().pr().is_pr_1() {
                return Err(EnableError::Absent)
            }
            // if the port is being used by another core, throw an error.
            // That means, software on another core has already configured the clocking
            // options of this peripheral.
            // For example, if your CPU software is the first to occupy this peripheral,
            // the INUSE bit would return 0, and settings to CGC bit would success. But
            // if it's not your CPU first to occupy, the INUSE bit would return 1 meaning
            // that this port is somehow locked by other CPUs.
            if self.reg().read().inuse().is_inuse_1() {
                return Err(EnableError::InUse)
            }
            // enable port clock
            self.reg().write(|w| w.cgc().set_bit());
            Ok(())
        }
        pub(crate) fn disable(&self) {
            // release port clock
            self.reg().write(|w| w.cgc().clear_bit());
        }
    }
)+
    };
}

pcc_impl! {
    LPUART0, lpuart0, PCC_LPUART0, pcc_lpuart0, "Low-Power UART";
    // LPUART1, lpuart1, PCC_LPUART1, pcc_lpuart1, "Low-Power UART";
    // LPUART2, lpuart2, PCC_LPUART2, pcc_lpuart2, "Low-Power UART";
    PORTA, porta, PCC_PORTA, pcc_porta, "Port";
    PORTB, portb, PCC_PORTB, pcc_portb, "Port";
    PORTC, portc, PCC_PORTC, pcc_portc, "Port";
    PORTD, portd, PCC_PORTD, pcc_portd, "Port";
}

/// Error that may occur when enabling the peripheral
#[derive(Clone, Copy, Debug)]
pub enum EnableError {
    /// The peripheral is not present on this device.
    Absent,
    /// Peripheral is being used by another core.
    InUse,
}
