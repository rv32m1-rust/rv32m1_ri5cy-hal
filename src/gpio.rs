//! General-Purpose Input/Output (GPIO)

use crate::port::*;
use core::marker::PhantomData;
use core::convert::Infallible;
use embedded_hal::digital::{OutputPin, StatefulOutputPin, ToggleableOutputPin, InputPin};

/// Gpio wrapper
pub struct Gpio<PIN, MODE> {
    pin: PIN,
    _mode: PhantomData<MODE>,
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

/// Port pin that is configured into ALT1 function
///
/// This trait should only be implemented by this HAL crate; user should not implement this trait.
/// Application users should call functions provided by `Gpio` struct.
pub trait Alt1Pin {
    #[doc(hidden)]
    type Gpio;
    #[doc(hidden)]
    fn configure_push_pull_output(self, gpio: &mut Self::Gpio) -> Self;
    #[doc(hidden)]
    fn configure_open_drain_output(self, gpio: &mut Self::Gpio) -> Self;
    #[doc(hidden)]
    fn configure_floating_input(self, gpio: &mut Self::Gpio) -> Self;
    #[doc(hidden)]
    fn configure_pull_up_input(self, gpio: &mut Self::Gpio) -> Self;
    #[doc(hidden)]
    fn configure_pull_down_input(self, gpio: &mut Self::Gpio) -> Self;
    #[doc(hidden)]
    fn set_low(&self);
    #[doc(hidden)]
    fn set_high(&self);
    #[doc(hidden)]
    fn is_set_high(&self) -> bool;
    #[doc(hidden)]
    fn toggle(&self);
    #[doc(hidden)]
    fn is_high(&self) -> bool;
}

impl<PIN: Alt1Pin, MODE> Gpio<PIN, MODE> {
    /// Configures the pin to operate as a push-pull output pin.
    pub fn into_push_pull_output(self, gpio: &mut PIN::Gpio) -> Gpio<PIN, Output<PushPull>> {
        Gpio { pin: self.pin.configure_push_pull_output(gpio), _mode: PhantomData }
    }
    /// Configures the pin to operate as an open-drain output pin.
    pub fn into_open_drain_output(self, gpio: &mut PIN::Gpio) -> Gpio<PIN, Output<OpenDrain>> {
        Gpio { pin: self.pin.configure_open_drain_output(gpio), _mode: PhantomData }
    }
    /// Configures the pin to operate as a floating input pin.
    pub fn into_floating_input(self, gpio: &mut PIN::Gpio) -> Gpio<PIN, Input<Floating>> {
        Gpio { pin: self.pin.configure_floating_input(gpio), _mode: PhantomData }
    }
    /// Configures the pin to operate as a pull-up input pin.
    pub fn into_pull_up_input(self, gpio: &mut PIN::Gpio) -> Gpio<PIN, Input<PullUp>> {
        Gpio { pin: self.pin.configure_pull_up_input(gpio), _mode: PhantomData }
    }
    /// Configures the pin to operate as a pull-down input pin.
    pub fn into_pull_down_input(self, gpio: &mut PIN::Gpio) -> Gpio<PIN, Input<PullDown>> {
        Gpio { pin: self.pin.configure_pull_down_input(gpio), _mode: PhantomData }
    }
}

impl<PIN: Alt1Pin, MODE> OutputPin for Gpio<PIN, Output<MODE>> {
    type Error = Infallible;

    fn try_set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low();
        Ok(())
    }

    fn try_set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high();
        Ok(())
    }
}

impl<PIN: Alt1Pin, MODE> StatefulOutputPin for Gpio<PIN, Output<MODE>> {
    fn try_is_set_high(&self) -> Result<bool, Infallible> {
        Ok(self.pin.is_set_high())
    }

    fn try_is_set_low(&self) -> Result<bool, Infallible> {
        Ok(!self.pin.is_set_high())
    }
}

impl<PIN: Alt1Pin, MODE> ToggleableOutputPin for Gpio<PIN, Output<MODE>> {
    type Error = Infallible;

    fn try_toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle();
        Ok(())
    }
}

impl<PIN: Alt1Pin, MODE> InputPin for Gpio<PIN, Input<MODE>> {
    type Error = Infallible;

    fn try_is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_set_high())
    }

    fn try_is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.pin.is_set_high())
    }
}

impl<PIN: Alt1Pin> InputPin for Gpio<PIN, Output<OpenDrain>> {
    type Error = Infallible;

    fn try_is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_set_high())
    }

    fn try_is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.pin.is_set_high())
    }
}

macro_rules! gpio_impl {
    ($GPIOX: ident, $gpiox: ident, $gpioy: ident, $portx: ident, [
        $( $PTXi: ident: $i: expr, )+
    ]) => {
mod $gpiox {
    use crate::port::$portx::*;
    use super::{Gpio, Output, PushPull, OpenDrain, Input, Floating, PullUp, PullDown};
    use super::ALT1;
    use crate::pac;
    use core::marker::PhantomData;

    #[inline] fn modify_gpio_direction_out($gpiox: &mut pac::$GPIOX, idx: usize) {
        $gpiox.pddr.modify(|r, w| unsafe { 
            w.pdd().bits(r.pdd().bits() | (1 << idx))
        });
    }

    #[inline] fn modify_gpio_direction_in($gpiox: &mut pac::$GPIOX, idx: usize) {
        $gpiox.pddr.modify(|r, w| unsafe { 
            w.pdd().bits(r.pdd().bits() & !(1 << idx))
        });
    }

    const GPIO_PTR: *const pac::$gpioy::RegisterBlock = pac::$GPIOX::ptr();

$(
    impl<AF> $PTXi<AF> {
        /// Configures the pin to operate as a push-pull output pin.
        pub fn into_push_pull_output(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Output<PushPull>> {
            modify_gpio_direction_out($gpiox, $i);
            let pin = self.into_af1_no_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as an open-drain output pin.
        pub fn into_open_drain_output(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Output<OpenDrain>> {
            modify_gpio_direction_out($gpiox, $i);
            let pin = self.into_af1_with_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a floating input pin.
        pub fn into_floating_input(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Input<Floating>> {
            modify_gpio_direction_in($gpiox, $i);
            let pin = self.into_af1_no_pull();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-up input pin.
        pub fn into_pull_up_input(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Input<PullUp>> {
            modify_gpio_direction_in($gpiox, $i);
            let pin = self.into_af1_pull_up();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-down input pin.
        pub fn into_pull_down_input(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Input<PullDown>> {
            modify_gpio_direction_in($gpiox, $i);
            let pin = self.into_af1_pull_down();
            Gpio { pin, _mode: PhantomData }
        }
    }

    impl super::Alt1Pin for $PTXi<ALT1> {
        type Gpio = pac::$GPIOX;
        fn configure_push_pull_output(self, gpio: &mut Self::Gpio) -> Self {
            modify_gpio_direction_out(gpio, $i);
            self.into_af1_no_open_drain()
        }
        fn configure_open_drain_output(self, gpio: &mut Self::Gpio) -> Self {
            modify_gpio_direction_out(gpio, $i);
            self.into_af1_with_open_drain()
        }
        fn configure_floating_input(self, gpio: &mut Self::Gpio) -> Self {
            modify_gpio_direction_in(gpio, $i);
            self.into_af1_no_pull()
        }
        fn configure_pull_up_input(self, gpio: &mut Self::Gpio) -> Self {
            modify_gpio_direction_in(gpio, $i);
            self.into_af1_pull_up()
        }
        fn configure_pull_down_input(self, gpio: &mut Self::Gpio) -> Self {
            modify_gpio_direction_in(gpio, $i);
            self.into_af1_pull_down()
        }
        fn set_low(&self) {
            unsafe { &*GPIO_PTR }.pcor.write(|w| unsafe { w.ptco().bits(1 << $i) });
        }
        fn set_high(&self) {
            unsafe { &*GPIO_PTR }.psor.write(|w| unsafe { w.ptso().bits(1 << $i) });
        }
        fn is_set_high(&self) -> bool {
            unsafe { &*GPIO_PTR }.pdor.read().bits() & (1 << $i) != 0
        }
        fn toggle(&self) {
            unsafe { &*GPIO_PTR }.ptor.write(|w| unsafe { w.ptto().bits(1 << $i) });
        }
        fn is_high(&self) -> bool {
            unsafe { &*GPIO_PTR }.pdir.read().bits() & (1 << $i) != 0
        }
    }
)+
}
    };
}

macro_rules! pfe_impl {
    ($($PTXi: ident:($pcri: ident, $PORTX: ident, $portx: ident),)+) => {
$(
    // not all pins support passive filter
    impl<MODE> Gpio<crate::port::$portx::$PTXi<ALT1>, Input<MODE>> {
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
    ($($PTXi: ident:($pcri: ident, $PORTX: ident, $portx: ident),)+) => {
$(
    // not all pins support drive strength config
    impl<MODE> Gpio<crate::port::$portx::$PTXi<ALT1>, Output<MODE>> {
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

gpio_impl! { GPIOA, gpioa, gpioa, porta, [
    PTA0: 0,
    PTA1: 1,
    PTA2: 2,
    PTA3: 3,
    PTA4: 4,
    PTA9: 9,
    PTA10: 10,
    PTA14: 14,
    PTA15: 15,
    PTA17: 17,
    PTA18: 18,
    PTA19: 19,
    PTA20: 20,
    PTA21: 21,
    PTA22: 22,
    PTA23: 23,
    PTA24: 24,
    PTA25: 25,
    PTA26: 26,
    PTA27: 27,
    PTA28: 28,
    PTA30: 30,
    PTA31: 31,
] }

gpio_impl! { GPIOB, gpiob, gpioa, portb, [
    PTB0: 0,
    PTB1: 1,
    PTB2: 2,
    PTB3: 3,
    PTB4: 4,
    PTB5: 5,
    PTB6: 6,
    PTB7: 7,
    PTB8: 8,
    PTB9: 9,
    PTB11: 11,
    PTB12: 12,
    PTB13: 13,
    PTB14: 14,
    PTB15: 15,
    PTB16: 16,
    PTB17: 17,
    PTB18: 18,
    PTB19: 19,
    PTB20: 20,
    PTB21: 21,
    PTB22: 22,
    PTB24: 24,
    PTB25: 25,
    PTB26: 26,
    PTB28: 28,
    PTB29: 29,
    PTB30: 30,
    PTB31: 31,
] }

gpio_impl! { GPIOC, gpioc, gpioa, portc, [
    PTC0: 0,
    PTC1: 1,
    PTC7: 7,
    PTC8: 8,
    PTC9: 9,
    PTC10: 10,
    PTC11: 11,
    PTC12: 12,
    // PTC26: 26, // see port.rs
    PTC27: 27,
    PTC28: 28,
    PTC29: 29,
    PTC30: 30,
] }

gpio_impl! { GPIOD, gpiod, gpioa, portd, [
    PTD0: 0,
    PTD1: 1,
    PTD2: 2,
    PTD3: 3,
    PTD4: 4,
    PTD5: 5,
    PTD6: 6,
    PTD7: 7,
    PTD8: 8,
    PTD9: 9,
    PTD10: 10,
    PTD11: 11,
] }

pfe_impl! {
    PTA0: (pcr0, PORTA, porta),
}

dse_impl! {
    PTC7: (pcr7, PORTC, portc),
    PTC8: (pcr8, PORTC, portc),
    PTC9: (pcr9, PORTC, portc),
    PTC10: (pcr10, PORTC, portc),
    PTC11: (pcr11, PORTC, portc),
    PTC12: (pcr12, PORTC, portc),
    PTD8: (pcr8, PORTD, portd),
    PTD9: (pcr9, PORTD, portd),
    PTD10: (pcr10, PORTD, portd),
    PTD11: (pcr11, PORTD, portd),
}
