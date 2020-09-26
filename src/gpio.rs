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

mod gpioa {
    use crate::port::porta::*;
    use super::{Gpio, Output, PushPull, OpenDrain, Input, Floating, PullUp, PullDown};
    use super::ALT1;
    use crate::pac;
    use core::marker::PhantomData;

    #[inline] fn modify_gpioa_direction_out(gpioa: &mut pac::GPIOA, idx: usize) {
        gpioa.pddr.modify(|r, w| unsafe { 
            w.pdd().bits(r.pdd().bits() | (1 << idx))
        });
    }

    #[inline] fn modify_gpioa_direction_in(gpioa: &mut pac::GPIOA, idx: usize) {
        gpioa.pddr.modify(|r, w| unsafe { 
            w.pdd().bits(r.pdd().bits() & !(1 << idx))
        });
    }

    impl<AF> PTA23<AF> {
        /// Configures the pin to operate as a push-pull output pin.
        pub fn into_push_pull_output(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Output<PushPull>> {
            modify_gpioa_direction_out(gpioa, 23);
            let pin = self.into_af1_no_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as an open-drain output pin.
        pub fn into_open_drain_output(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Output<OpenDrain>> {
            modify_gpioa_direction_out(gpioa, 23);
            let pin = self.into_af1_with_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a floating input pin.
        pub fn into_floating_input(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Input<Floating>> {
            modify_gpioa_direction_in(gpioa, 23);
            let pin = self.into_af1_no_pull();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-up input pin.
        pub fn into_pull_up_input(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Input<PullUp>> {
            modify_gpioa_direction_in(gpioa, 23);
            let pin = self.into_af1_pull_up();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-down input pin.
        pub fn into_pull_down_input(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Input<PullDown>> {
            modify_gpioa_direction_in(gpioa, 23);
            let pin = self.into_af1_pull_down();
            Gpio { pin, _mode: PhantomData }
        }
    }

    impl<MODE> Gpio<PTA23<ALT1>, MODE> {
        /// Configures the pin to operate as a push-pull output pin.
        pub fn into_push_pull_output(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Output<PushPull>> {
            modify_gpioa_direction_out(gpioa, 23);
            let pin = self.pin.into_af1_no_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as an open-drain output pin.
        pub fn into_open_drain_output(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Output<OpenDrain>> {
            modify_gpioa_direction_out(gpioa, 23);
            let pin = self.pin.into_af1_with_open_drain();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a floating input pin.
        pub fn into_floating_input(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Input<Floating>> {
            modify_gpioa_direction_in(gpioa, 23);
            let pin = self.pin.into_af1_no_pull();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-up input pin.
        pub fn into_pull_up_input(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Input<PullUp>> {
            modify_gpioa_direction_in(gpioa, 23);
            let pin = self.pin.into_af1_pull_up();
            Gpio { pin, _mode: PhantomData }
        }
        /// Configures the pin to operate as a pull-down input pin.
        pub fn into_pull_down_input(self, gpioa: &mut pac::GPIOA) -> Gpio<PTA23<ALT1>, Input<PullDown>> {
            modify_gpioa_direction_in(gpioa, 23);
            let pin = self.pin.into_af1_pull_down();
            Gpio { pin, _mode: PhantomData }
        }
    }
}
