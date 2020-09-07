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
        pub(crate) fn reg(&self) -> &pcc0::$PCC_REGX {
            unsafe { &(*PCC0::ptr()).$pcc_regx }
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

// todo: change a name
#[derive(Clone, Copy, Debug)]
pub enum EnableError {
    Absent,
    InUse,
}
