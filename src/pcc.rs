//! Peripheral Clock Controller (PCC)

use crate::pac::{pcc0, PCC0};

pub trait PccExt {
    fn constrain(self) -> Pcc;
}

macro_rules! port_impl {
    ($($PORTX: ident, $portx: ident, $PCC_PORTX: ident, $pcc_portx: ident;)+) => {
impl PccExt for PCC0 {
    fn constrain(self) -> Pcc {
        Pcc {
            $( $portx: $PORTX { _ownership: () }, )+
        }
    }
}

pub struct Pcc {
    $( 
        /// Port
        pub $portx: $PORTX,
    )+
}
$(
    pub struct $PORTX {
        _ownership: ()
    }

    impl $PORTX {
        pub fn port(&self) -> &pcc0::$PCC_PORTX {
            unsafe { &(*PCC0::ptr()).$pcc_portx }
        }
    }
)+
    };
}

port_impl! {
    PORTA, porta, PCC_PORTA, pcc_porta;
    PORTB, portb, PCC_PORTB, pcc_portb;
    PORTC, portc, PCC_PORTC, pcc_portc;
    PORTD, portd, PCC_PORTD, pcc_portd;
}
