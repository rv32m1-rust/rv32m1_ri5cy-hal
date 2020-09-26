//! Port peripheral

use crate::pcc::EnableError;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait PortExt {
    /// The port clock controller that controls this port
    type Clock;

    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    ///
    /// It's possible to have errors because this GPIO peripheral may be in use
    /// by another core, or the peripheral is absent on this device.
    fn split(self, pcc_port: &mut Self::Clock) -> Result<Self::Parts, EnableError>;
}

/// Alternate function 0 (type state)
pub struct ALT0;

/// Alternate function 1 (type state)
pub struct ALT1;

/// Alternate function 2 (type state)
pub struct ALT2;

/// Alternate function 3 (type state)
pub struct ALT3;

/// Alternate function 4 (type state)
pub struct ALT4;

/// Alternate function 5 (type state)
pub struct ALT5;

/// Alternate function 6 (type state)
pub struct ALT6;

/// Alternate function 7 (type state)
pub struct ALT7;

pub mod porta {
    use super::{ALT0, ALT1, ALT2, ALT3, ALT4, ALT5, ALT6, ALT7};
    use core::marker::PhantomData;
    use crate::pac;

    pub struct Parts {
        pub pta23: PTA23<ALT0>,
    }

    pub struct PTA23<AF> {
        pcr: pac::porta::PCR23,
        _function: PhantomData<AF>,
    }

    impl<AF> PTA23<AF> {
        /// Configures the pin to operate as alternate function 0
        pub fn into_af0(self) -> PTA23<ALT0> { // on some pins this is named into_disabled
            self.pcr.write(|w| 
                w.mux().mux_0() // Pin Mux Control: ALT0
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        
        /* ALT1: please use into_push_pull_output etc in gpio modules */
        
        /// Configures the pin to operate as alternate function 2
        pub fn into_af2(self) -> PTA23<ALT2> {
            self.pcr.write(|w| 
                w.mux().mux_2() // Pin Mux Control: ALT2
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 3
        pub fn into_af3(self) -> PTA23<ALT3> {
            self.pcr.write(|w| 
                w.mux().mux_3() // Pin Mux Control: ALT3
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 4
        pub fn into_af4(self) -> PTA23<ALT4> {
            self.pcr.write(|w| 
                w.mux().mux_4() // Pin Mux Control: ALT4
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 5
        pub fn into_af5(self) -> PTA23<ALT5> {
            self.pcr.write(|w| 
                w.mux().mux_5() // Pin Mux Control: ALT5
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 6
        pub fn into_af6(self) -> PTA23<ALT6> {
            self.pcr.write(|w| 
                w.mux().mux_6() // Pin Mux Control: ALT6
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 7
        pub fn into_af7(self) -> PTA23<ALT7> {
            self.pcr.write(|w| 
                w.mux().mux_7() // Pin Mux Control: ALT7
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
    }

    impl<AF> PTA23<AF> {
        #[inline] pub(crate) fn into_af1_no_open_drain(self) -> PTA23<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .ode().clear_bit() // Open Drain Enable: 0
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_with_open_drain(self) -> PTA23<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .ode().set_bit() // Open Drain Enable: 1
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_no_pull(self) -> PTA23<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().clear_bit() // Pull Enable: 0
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_pull_up(self) -> PTA23<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().set_bit() // Pull Enable: 1
                .ps().set_bit() // Pull Select: 1 (pullup)
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_pull_down(self) -> PTA23<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().set_bit() // Pull Enable: 1
                .ps().clear_bit() // Pull Select: 0 (pulldown)
            );
            PTA23 { pcr: self.pcr, _function: PhantomData }
        }
    }

}
