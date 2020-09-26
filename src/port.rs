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

macro_rules! port_impl {
    ($PORTX: ident, $portx: ident, $PTXx: ident, [
        $($PTXi: ident:($ptxi: ident, $i: expr, $PCRi: ident, $mode: ty),)+
    ]) => {
/// Port
pub mod $portx {
    use super::{PortExt, ALT0, ALT1, ALT2, ALT3, ALT4, ALT5, ALT6, ALT7};
    use core::marker::PhantomData;
    use core::mem::MaybeUninit;
    use crate::{pac, pcc};
    use crate::pcc::EnableError;

    impl PortExt for pac::$PORTX {
        type Clock = pcc::$PORTX;

        type Parts = Parts;

        fn split(self, pcc_port: &mut pcc::$PORTX) -> Result<Self::Parts, EnableError> {
            pcc_port.try_enable()?;
            Ok(Parts {
                $(
                    $ptxi: $PTXi { pcr: unsafe { MaybeUninit::uninit().assume_init() }, _function: PhantomData },
                )+
            })
        }
    }

    impl Parts {
        /// Free and release the port taken so it could be used by another core.
        ///
        /// Users may use this function like:
        /// ```
        /// let pta0 = gpioa.pta0.into_push_pull_output();
        /// let pta24 = gpioa.pta24.into_push_pull_output();
        /// // code that uses pin pta0 and pta24 etc.
        /// // switch the port mode back before free operation.
        /// Parts {
        ///     pta0: pta0.into_floating_input(),
        ///     pta24: pta24.into_floating_input(),
        /// ..parts }.free(); // free this port
        /// ```
        pub fn free(self, pcc_port: &mut pcc::$PORTX) -> pac::$PORTX {
            use core::mem::transmute;
            // disable peripheral clock
            pcc_port.disable();
            // return the ownership of $PORTX
            unsafe { transmute(()) }
        }
    }

    /// Port parts
    pub struct Parts {
        $( pub $ptxi: $PTXi<ALT0>, )+
    }

$(
    /// Pin
    pub struct $PTXi<AF> {
        pcr: pac::$portx::$PCRi,
        _function: PhantomData<AF>,
    }

    impl<AF> $PTXi<AF> {
        /// Configures the pin to operate as alternate function 0
        pub fn into_af0(self) -> $PTXi<ALT0> { // on some pins this is named into_disabled
            self.pcr.write(|w| 
                w.mux().mux_0() // Pin Mux Control: ALT0
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        
        /* ALT1: please use into_push_pull_output etc in gpio modules */
        
        /// Configures the pin to operate as alternate function 2
        pub fn into_af2(self) -> $PTXi<ALT2> {
            self.pcr.write(|w| 
                w.mux().mux_2() // Pin Mux Control: ALT2
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 3
        pub fn into_af3(self) -> $PTXi<ALT3> {
            self.pcr.write(|w| 
                w.mux().mux_3() // Pin Mux Control: ALT3
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 4
        pub fn into_af4(self) -> $PTXi<ALT4> {
            self.pcr.write(|w| 
                w.mux().mux_4() // Pin Mux Control: ALT4
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 5
        pub fn into_af5(self) -> $PTXi<ALT5> {
            self.pcr.write(|w| 
                w.mux().mux_5() // Pin Mux Control: ALT5
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 6
        pub fn into_af6(self) -> $PTXi<ALT6> {
            self.pcr.write(|w| 
                w.mux().mux_6() // Pin Mux Control: ALT6
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        /// Configures the pin to operate as alternate function 7
        pub fn into_af7(self) -> $PTXi<ALT7> {
            self.pcr.write(|w| 
                w.mux().mux_7() // Pin Mux Control: ALT7
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
    }

    impl<AF> $PTXi<AF> {
        #[inline] pub(crate) fn into_af1_no_open_drain(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .ode().clear_bit() // Open Drain Enable: 0
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_with_open_drain(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .ode().set_bit() // Open Drain Enable: 1
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_no_pull(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().clear_bit() // Pull Enable: 0
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_pull_up(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().set_bit() // Pull Enable: 1
                .ps().set_bit() // Pull Select: 1 (pullup)
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_pull_down(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().set_bit() // Pull Enable: 1
                .ps().clear_bit() // Pull Select: 0 (pulldown)
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
    }
)+
}
    };
}

port_impl! { PORTA, porta, PTAx, [
    PTA0: (pta0, 0, PCR0, AF0),
    PTA1: (pta1, 1, PCR1, AF0),
    PTA2: (pta2, 2, PCR2, AF0),
    PTA3: (pta3, 3, PCR3, AF0),
    PTA4: (pta4, 4, PCR4, AF0),
    PTA9: (pta9, 9, PCR9, AF0),
    PTA10: (pta10, 10, PCR10, AF0),
    PTA14: (pta14, 14, PCR14, AF0),
    PTA15: (pta15, 15, PCR15, AF0),
    PTA17: (pta17, 17, PCR17, AF0),
    PTA18: (pta18, 18, PCR18, AF0),
    PTA19: (pta19, 19, PCR19, AF0),
    PTA20: (pta20, 20, PCR20, AF0),
    PTA21: (pta21, 21, PCR21, AF0),
    PTA22: (pta22, 22, PCR22, AF0),
    PTA23: (pta23, 23, PCR23, AF0),
    PTA24: (pta24, 24, PCR24, AF0),
    PTA25: (pta25, 25, PCR25, AF0),
    PTA26: (pta26, 26, PCR26, AF0),
    PTA27: (pta27, 27, PCR27, AF0),
    PTA28: (pta28, 28, PCR28, AF0),
    PTA30: (pta30, 30, PCR30, AF0),
    PTA31: (pta31, 31, PCR31, AF0),
] }
