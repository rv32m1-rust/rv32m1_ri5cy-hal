//! Peripheral Clock Controller (PCC)

use crate::pac::{pcc0, PCC0};

pub trait PccExt {
    fn constrain(self) -> Pcc;
}

impl PccExt for PCC0 {
    fn constrain(self) -> Pcc {
        Pcc {
            porta: PORTA { _ownership: () },
        }
    }
}

pub struct Pcc {
    pub porta: PORTA,
}

pub struct PORTA {
    _ownership: ()
}

impl PORTA {
    pub fn port(&self) -> &pcc0::PCC_PORTA {
        unsafe { &(*PCC0::ptr()).pcc_porta }
    }
}
