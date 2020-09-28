//! General-Purpose Input/Output (GPIO)

use crate::port::*;
use core::marker::PhantomData;

/// Gpio wrapper
pub struct Gpio<PIN, MODE> {
    pin: PIN,
    _mode: PhantomData<MODE>,
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

macro_rules! gpio_impl {
    ($GPIOX: ident, $gpiox: ident, $portx: ident, [
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

    impl<MODE> Gpio<$PTXi<ALT1>, MODE> {
        /// Configures the pin to operate as a push-pull output pin.
        pub fn into_push_pull_output(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Output<PushPull>> {
            modify_gpio_direction_out($gpiox, $i);
            let pin = self.pin.into_af1_no_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as an open-drain output pin.
        pub fn into_open_drain_output(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Output<OpenDrain>> {
            modify_gpio_direction_out($gpiox, $i);
            let pin = self.pin.into_af1_with_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a floating input pin.
        pub fn into_floating_input(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Input<Floating>> {
            modify_gpio_direction_in($gpiox, $i);
            let pin = self.pin.into_af1_no_pull();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-up input pin.
        pub fn into_pull_up_input(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Input<PullUp>> {
            modify_gpio_direction_in($gpiox, $i);
            let pin = self.pin.into_af1_pull_up();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-down input pin.
        pub fn into_pull_down_input(self, $gpiox: &mut pac::$GPIOX) -> Gpio<$PTXi<ALT1>, Input<PullDown>> {
            modify_gpio_direction_in($gpiox, $i);
            let pin = self.pin.into_af1_pull_down();
            Gpio { pin, _mode: PhantomData }
        }
    }
)+
}
    };
}

gpio_impl! { GPIOA, gpioa, porta, [
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

gpio_impl! { GPIOB, gpiob, portb, [
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

gpio_impl! { GPIOC, gpioc, portc, [
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

gpio_impl! { GPIOD, gpiod, portd, [
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
