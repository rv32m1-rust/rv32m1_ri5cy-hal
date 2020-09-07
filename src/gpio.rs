//! General-Purpose Input/Output (GPIO)

/*  todo: interrupts, both flags and settings.
    do interrupt flag and settings freeze when pin is locked? */
/*
    todo: there is a design bug. after pin locking, the input/output
    state can still be changed, (but mode of input and output cannot)
    e.g. set a pin to Input<Floating> and Output<OpenDrain>, after locked
    we switch between two, but cannot change it into Input<PullUp> etc.
*/
/*
    This chip supports BATCH modifications of port settings;
    hal designs should take best use of this feature.
    (luojia65)
*/

use core::marker::PhantomData;
use crate::pcc::EnableError;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait GpioExt {
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

/// Pin slew rate, valid in all digital pin muxing modes.
#[derive(Clone, Copy, Debug)]
pub enum SlewRate {
    /// Fast slew rate is configured on the corresponding pin,
    /// if the pin is configured as a digital output.
    Fast,
    /// Slow slew rate is configured on the corresponding pin,
    /// if the pin is configured as a digital output.
    Slow,
}

/// Pin drive strength, valid in all digital pin muxing modes.
#[derive(Clone, Copy, Debug)]
pub enum DriveStrength {
    /// Low drive strength is configured on the corresponding pin,
    /// if pin is configured as a digital output.
    Low,
    /// High drive strength is configured on the corresponding pin,
    /// if pin is configured as a digital output.
    High,
}

/// General-purpose input, for the GPIO function (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Hi-Z Floating input (type state)
pub struct Floating;

/// Pull-up input (type state)
pub struct PullUp;

/// Pull-down input (type state)
pub struct PullDown;

/// General-purpose output, for the GPIO function (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push-pull output (type state)
pub struct PushPull;

/// Open-drain output (type state)
pub struct OpenDrain;

/// Wraps a pin if its Pin Control Register (PCR) is locked
pub struct Locked<T>(T);

// implement all digital input/output traits for locked pins
//
// there could be a better design: impl Deref and DerefMut for Locked<_>. 
// But as soon as we have slew rate configuration function which requires a 
// &mut self and cannot be changed after locked, we dropped this design.
// If you have any good idea, please fire an issue or raise a pull request.
mod impl_for_locked {
    use super::Locked;
    use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

    impl<T> OutputPin for Locked<T>
    where
        T: OutputPin,
    {
        type Error = T::Error;

        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            self.0.try_set_low()
        }

        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            self.0.try_set_high()
        }
    }

    impl<T> StatefulOutputPin for Locked<T>
    where
        T: StatefulOutputPin,
    {
        fn try_is_set_high(&self) -> Result<bool, Self::Error> {
            self.0.try_is_set_high()
        }

        fn try_is_set_low(&self) -> Result<bool, Self::Error> {
            self.0.try_is_set_low()
        }
    }

    impl<T> ToggleableOutputPin for Locked<T>
    where
        T: ToggleableOutputPin,
    {
        type Error = T::Error;

        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            self.0.try_toggle()
        }
    }

    impl<T> InputPin for Locked<T>
    where
        T: InputPin,
    {
        type Error = T::Error;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            self.0.try_is_high()
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            self.0.try_is_low()
        }
    }
}

macro_rules! gpio_impl {
    ($GPIOX: ident, $gpiox: ident, $gpioy: ident, $PORTX: ident, $portx: ident, $PTXx: ident, [
        $($PTXi: ident:($ptxi: ident, $i: expr, $pcri: ident, $mode: ty),)+
    ]) => {
/// GPIO
pub mod $gpiox {
    use crate::{pac, pcc};
    use core::{convert::Infallible, marker::PhantomData};
    use super::{
        GpioExt, EnableError, Locked, Output, OpenDrain, PushPull, Input, Floating,
        PullUp, PullDown, SlewRate
    };
    // use super::DriveStrength;
    use embedded_hal::digital::{OutputPin, StatefulOutputPin, ToggleableOutputPin, InputPin};
    use riscv::interrupt;

    const GPIO_PTR: *const pac::$gpioy::RegisterBlock = pac::$GPIOX::ptr();

    const PORT_PTR: *const pac::$portx::RegisterBlock = pac::$PORTX::ptr();

    impl GpioExt for (pac::$GPIOX, pac::$PORTX) {
        type Clock = pcc::$PORTX;

        type Parts = Parts;

        fn split(self, pcc_port: &mut pcc::$PORTX) -> Result<Self::Parts, EnableError> {
            interrupt::free(|_| {
                pcc_port.try_enable()?;
                Ok(Parts {
                    $( $ptxi: $PTXi { _mode: PhantomData }, )+
                })
            })
        }
    }

    /// GPIO parts
    pub struct Parts {
    $(
        /// Pin
        pub $ptxi: $PTXi<$mode>,
    )+
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
        pub fn free(self, pcc_port: &mut pcc::$PORTX) -> (pac::$GPIOX, pac::$PORTX) {
            use core::mem::transmute;
            // disable peripheral clock
            pcc_port.disable();
            // return the ownership of $GPIOX and $PORTX
            unsafe { (transmute(()), transmute(())) }
        }
    }

    /// Partially erased pin
    pub struct $PTXx<MODE> {
        i: u8,
        _mode: PhantomData<MODE>,
    }

    impl<MODE> $PTXx<MODE> {
        #[inline]
        fn pin_mask(&self) -> u32 {
            1 << self.i
        }
    }

    impl<MODE> OutputPin for $PTXx<Output<MODE>> {
        type Error = Infallible;
        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.pcor.write(|w| unsafe { w.ptco().bits(self.pin_mask()) });
            Ok(())
        }
        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.psor.write(|w| unsafe { w.ptso().bits(self.pin_mask()) });
            Ok(())
        }
    }

    impl<MODE> InputPin for $PTXx<Input<MODE>> {
        type Error = Infallible;
        fn try_is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() != 0)
        }
        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() == 0)
        }
    }

    impl<MODE> StatefulOutputPin for $PTXx<Output<MODE>> {
        fn try_is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & self.pin_mask() != 0)
        }
        fn try_is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & self.pin_mask() == 0)
        }
    }

    impl<MODE> ToggleableOutputPin for $PTXx<Output<MODE>> {
        type Error = Infallible;
        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.ptor.write(|w| unsafe { w.ptto().bits(self.pin_mask()) });
            Ok(())
        }
    }

    impl InputPin for $PTXx<Output<OpenDrain>> {
        type Error = Infallible;
        fn try_is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() != 0)
        }
        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() == 0)
        }
    }
$(
    /// Pin
    pub struct $PTXi<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl<MODE> $PTXi<MODE> {
        /// Configures the pin to operate as a push-pull output pin.
        pub fn into_push_pull_output(self) -> $PTXi<Output<PushPull>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).$pcri.write(|w| w.mux().mux_1().ode().clear_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() | (1 << $i)));
            });
            $PTXi { _mode: PhantomData }
        }

        /// Configures the pin to operate as an open-drain output pin.
        pub fn into_open_drain_output(self) -> $PTXi<Output<OpenDrain>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).$pcri.write(|w| w.mux().mux_1().ode().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() | (1 << $i)));
            });
            $PTXi { _mode: PhantomData }
        }

        /// Configures the pin to operate as a floating input pin.
        pub fn into_floating_input(self) -> $PTXi<Input<Floating>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).$pcri.write(|w| w.mux().mux_1().pe().clear_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << $i)));
            });
            $PTXi { _mode: PhantomData }
        }

        /// Configures the pin to operate as a pull-up input pin.
        pub fn into_pull_up_input(self) -> $PTXi<Input<PullUp>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).$pcri.write(|w| w.mux().mux_1().ps().set_bit().pe().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << $i)));
            });
            $PTXi { _mode: PhantomData }
        }

        /// Configures the pin to operate as a pull-down input pin.
        pub fn into_pull_down_input(self) -> $PTXi<Input<PullDown>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).$pcri.write(|w| w.mux().mux_1().ps().clear_bit().pe().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << $i)));
            });
            $PTXi { _mode: PhantomData }
        }
    }

    impl<MODE> $PTXi<Output<MODE>> {
        /// Change the pin's slew rate.
        pub fn set_slew_rate(&self, value: SlewRate) {
            unsafe { &*PORT_PTR }.$pcri.write(|w| match value {
                SlewRate::Fast => w.sre().clear_bit(),
                SlewRate::Slow => w.sre().set_bit(),
            });
        }
    }

    impl<MODE> $PTXi<MODE> {
        /// Erases the pin number from the type
        ///
        /// This is useful when you want to collect the pins into an array where you
        /// need all the elements to have the same type
        pub fn downgrade(self) -> $PTXx<MODE> {
            $PTXx {
                i: $i,
                _mode: PhantomData,
            }
        }
    }

    impl<MODE> $PTXi<MODE> {
        /// Lock this pin to deny any further mode updates until next system reset.
        ///
        /// After this operation, you are still allowed to change the output state or read
        /// from input state. However, it locks the pin mode so you cannot change pin mode
        /// anymore before system reset.
        ///
        /// This operation would lock the pin's Pin Control Register (PCR), which controls
        /// pin's internal pulls, open drain enables. The pin mode may still be changed, but
        /// only between given one output mode and one input mode specified by the PCR.
        /// We have not provided a function to switch between input and output after locked,
        /// and this is considered as a limit of this library. If you have any good idea,
        /// please [fire an issue] to tell us.
        ///
        /// [fire an issue]: https://github.com/rv32m1-rust/rv32m1_ri5cy-hal/issues/new
        pub fn lock(self) -> Locked<$PTXi<MODE>> {
            unsafe { &*PORT_PTR }.$pcri.write(|w| w.lk().set_bit());
            Locked(self)
        }
    }

    impl<MODE> OutputPin for $PTXi<Output<MODE>> {
        type Error = Infallible;

        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.pcor.write(|w| unsafe { w.ptco().bits(1 << $i) });
            Ok(())
        }

        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.psor.write(|w| unsafe { w.ptso().bits(1 << $i) });
            Ok(())
        }
    }

    impl<MODE> StatefulOutputPin for $PTXi<Output<MODE>> {
        fn try_is_set_high(&self) -> Result<bool, Infallible> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & (1 << $i) != 0)
        }

        fn try_is_set_low(&self) -> Result<bool, Infallible> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & (1 << $i) == 0)
        }
    }

    impl<MODE> ToggleableOutputPin for $PTXi<Output<MODE>> {
        type Error = Infallible;

        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.ptor.write(|w| unsafe { w.ptto().bits(1 << $i) });
            Ok(())
        }
    }

    impl<MODE> InputPin for $PTXi<Input<MODE>> {
        type Error = Infallible;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & (1 << $i) != 0)
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & (1 << $i) == 0)
        }
    }

    impl InputPin for $PTXi<Output<OpenDrain>> {
        type Error = Infallible;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & (1 << $i) != 0)
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & (1 << $i) == 0)
        }
    }
)+
}
    };
}

macro_rules! pfe_impl {
    ($($PTXi: ident:($pcri: ident, $PORTX: ident, $gpiox: ident),)+) => {
$(
    // not all pins support passive filter
    impl<MODE> $gpiox::$PTXi<Input<MODE>> {
        /// Enable or disable passive filter for this pin.
        ///
        /// Passive filter configuration is valid in all digital pin muxing modes.
        /// This function needs a mutable borrow of self to change its state register.
        pub fn set_passive_filter(&mut self, value: bool) {
            unsafe { &*crate::pac::$PORTX::ptr() }.$pcri.write(|w| match value {
                false => w.pfe().clear_bit(),
                true => w.pfe().set_bit(),
            });
        }
    }
)+
    };
}

macro_rules! dse_impl {
    ($($PTXi: ident:($pcri: ident, $PORTX: ident, $gpiox: ident),)+) => {
$(
    // not all pins support drive strength config
    impl<MODE> $gpiox::$PTXi<Output<MODE>> {
        /// Configure the drive strength on the corresponding pin.
        ///
        /// According to the chip referrence manual, the configuration is only valid
        /// if the pin is configured as a digital output.
        /// This function needs a mutable borrow of self to change its state register.
        pub fn set_drive_strength(&mut self, value: DriveStrength) {
            unsafe { &*crate::pac::$PORTX::ptr() }.$pcri.write(|w| match value {
                DriveStrength::Low => w.dse().clear_bit(),
                DriveStrength::High => w.dse().set_bit(),
            });
        }
    }
)+
    }
}

// todo: change all default mode into Analog<What>

gpio_impl! { GPIOA, gpioa, gpioa, PORTA, porta, PTAx, [
    PTA0: (pta0, 0, pcr0, Input<Floating>),
    PTA1: (pta1, 1, pcr1, Input<Floating>),
    PTA2: (pta2, 2, pcr2, Input<Floating>),
    PTA3: (pta3, 3, pcr3, Input<Floating>),
    PTA4: (pta4, 4, pcr4, Input<Floating>),
    PTA9: (pta9, 9, pcr9, Input<Floating>),
    PTA10: (pta10, 10, pcr10, Input<Floating>),
    PTA14: (pta14, 14, pcr14, Input<Floating>),
    PTA15: (pta15, 15, pcr15, Input<Floating>),
    PTA17: (pta17, 17, pcr17, Input<Floating>),
    PTA18: (pta18, 18, pcr18, Input<Floating>),
    PTA19: (pta19, 19, pcr19, Input<Floating>),
    PTA20: (pta20, 20, pcr20, Input<Floating>),
    PTA21: (pta21, 21, pcr21, Input<Floating>),
    PTA22: (pta22, 22, pcr22, Input<Floating>),
    PTA23: (pta23, 23, pcr23, Input<Floating>),
    PTA24: (pta24, 24, pcr24, Input<Floating>),
    PTA25: (pta25, 25, pcr25, Input<Floating>),
    PTA26: (pta26, 26, pcr26, Input<Floating>),
    PTA27: (pta27, 27, pcr27, Input<Floating>),
    PTA28: (pta28, 28, pcr28, Input<Floating>),
    PTA30: (pta30, 30, pcr30, Input<Floating>),
    PTA31: (pta31, 31, pcr31, Input<Floating>),
] }

gpio_impl! { GPIOB, gpiob, gpioa, PORTB, portb, PTCx, [
    PTB0: (ptb0, 0, pcr0, Input<Floating>),
    PTB1: (ptb1, 1, pcr1, Input<Floating>),
    PTB2: (ptb2, 2, pcr2, Input<Floating>),
    PTB3: (ptb3, 3, pcr3, Input<Floating>),
    PTB4: (ptb4, 4, pcr4, Input<Floating>),
    PTB5: (ptb5, 5, pcr5, Input<Floating>),
    PTB6: (ptb6, 6, pcr6, Input<Floating>),
    PTB7: (ptb7, 7, pcr7, Input<Floating>),
    PTB8: (ptb8, 8, pcr8, Input<Floating>),
    PTB9: (ptb9, 9, pcr9, Input<Floating>),
    PTB11: (ptb11, 11, pcr11, Input<Floating>),
    PTB12: (ptb12, 12, pcr12, Input<Floating>),
    PTB13: (ptb13, 13, pcr13, Input<Floating>),
    PTB14: (ptb14, 14, pcr14, Input<Floating>),
    PTB15: (ptb15, 15, pcr15, Input<Floating>),
    PTB16: (ptb16, 16, pcr16, Input<Floating>),
    PTB17: (ptb17, 17, pcr17, Input<Floating>),
    PTB18: (ptb18, 18, pcr18, Input<Floating>),
    PTB19: (ptb19, 19, pcr19, Input<Floating>),
    PTB20: (ptb20, 20, pcr20, Input<Floating>),
    PTB21: (ptb21, 21, pcr21, Input<Floating>),
    PTB22: (ptb22, 22, pcr22, Input<Floating>),
    PTB24: (ptb24, 24, pcr24, Input<Floating>),
    PTB25: (ptb25, 25, pcr25, Input<Floating>),
    PTB26: (ptb26, 26, pcr26, Input<Floating>),
    PTB28: (ptb28, 28, pcr28, Input<Floating>),
    PTB29: (ptb29, 29, pcr29, Input<Floating>),
    PTB30: (ptb30, 30, pcr30, Input<Floating>),
    PTB31: (ptb31, 31, pcr31, Input<Floating>),
] }

gpio_impl! { GPIOC, gpioc, gpioa, PORTC, portc, PTCx, [
    PTC0: (ptc0, 0, pcr0, Input<Floating>),
    PTC1: (ptc1, 1, pcr1, Input<Floating>),
    PTC7: (ptc7, 7, pcr7, Input<Floating>),
    PTC8: (ptc8, 8, pcr8, Input<Floating>),
    PTC9: (ptc9, 9, pcr9, Input<Floating>),
    PTC10: (ptc10, 10, pcr10, Input<Floating>),
    PTC11: (ptc11, 11, pcr11, Input<Floating>),
    PTC12: (ptc12, 12, pcr12, Input<Floating>),
    PTC26: (ptc26, 26, pcr26, Input<Floating>),
    PTC27: (ptc27, 27, pcr27, Input<Floating>),
    PTC28: (ptc28, 28, pcr28, Input<Floating>),
    PTC29: (ptc29, 29, pcr29, Input<Floating>),
    PTC30: (ptc30, 30, pcr30, Input<Floating>),
] }

gpio_impl! { GPIOD, gpiod, gpioa, PORTD, portd, PTCx, [
    PTD0: (ptd0, 0, pcr0, Input<Floating>),
    PTD1: (ptd1, 1, pcr1, Input<Floating>),
    PTD2: (ptd2, 2, pcr2, Input<Floating>),
    PTD3: (ptd3, 3, pcr3, Input<Floating>),
    PTD4: (ptd4, 4, pcr4, Input<Floating>),
    PTD5: (ptd5, 5, pcr5, Input<Floating>),
    PTD6: (ptd6, 6, pcr6, Input<Floating>),
    PTD7: (ptd7, 7, pcr7, Input<Floating>),
    PTD8: (ptd8, 8, pcr8, Input<Floating>),
    PTD9: (ptd9, 9, pcr9, Input<Floating>),
    PTD10: (ptd10, 10, pcr10, Input<Floating>),
    PTD11: (ptd11, 11, pcr11, Input<Floating>),
] }

pfe_impl! {
    PTA0: (pcr0, PORTA, gpioa),
}

dse_impl! {
    PTC7: (pcr7, PORTC, gpioc),
    PTC8: (pcr8, PORTC, gpioc),
    PTC9: (pcr9, PORTC, gpioc),
    PTC10: (pcr10, PORTC, gpioc),
    PTC11: (pcr11, PORTC, gpioc),
    PTC12: (pcr12, PORTC, gpioc),
    PTD8: (pcr8, PORTD, gpiod),
    PTD9: (pcr9, PORTD, gpiod),
    PTD10: (pcr10, PORTD, gpiod),
    PTD11: (pcr11, PORTD, gpiod),
}
