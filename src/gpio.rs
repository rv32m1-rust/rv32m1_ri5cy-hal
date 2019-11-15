/// General Purpose I/Os

use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait GpioExt {
    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Pin slew rate, valid in all digital pin muxing modes.
#[derive(Clone, Copy, Debug)]
pub enum SlewRate {
    Fast,
    Slow
}

/// Pin drive strength, valid in all digital pin muxing modes.
#[derive(Clone, Copy, Debug)]
pub enum DriveStrength {
    Low,
    High,
}

/// General-purpose input, for the GPIO function (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Hi-Z Floating input (type state)
pub struct Floating;

/// Pull up input (type state)
pub struct PullUp;

/// Pull down input (type state)
pub struct PullDown;

/// General-purpose output, for the GPIO function (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

// PCRx::DSE, Drive Strength Enable
// 0b&&output->low
// 1b&&output->high
// todo

/// Push pull output (type state)
pub struct PushPull;

/// Open drain output (type state)
pub struct OpenDrain;

/// Wraps a pin if its Pin Control Register (PCR) is locked 
pub struct Locked<T>(T);

mod impl_for_locked {
    use super::Locked;
    use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin, InputPin};

    impl<T> OutputPin for Locked<T> 
    where 
        T: OutputPin 
    {
        type Error = T::Error;

        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.0.set_low()
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.0.set_high()
        }
    }    
    
    impl<T> StatefulOutputPin for Locked<T> 
    where 
        T: StatefulOutputPin 
    {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            self.0.is_set_high()
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            self.0.is_set_low()
        }
    }

    impl<T> ToggleableOutputPin for Locked<T> 
    where 
        T: ToggleableOutputPin 
    {
        type Error = T::Error;
    
        fn toggle(&mut self) -> Result<(), Self::Error> {
            self.0.toggle()
        }
    }
    
    impl<T> InputPin for Locked<T> 
    where 
        T: InputPin 
    {   
        type Error = T::Error;
    
        fn is_high(&self) -> Result<bool, Self::Error> {
            self.0.is_high()
        }
    
        fn is_low(&self) -> Result<bool, Self::Error> {
            self.0.is_low()
        }
    }
}

pub mod gpioa {
    use crate::pac;
    use core::{convert::Infallible, marker::PhantomData};
    use super::{
        GpioExt, Locked, Output, OpenDrain, PushPull, Input, Floating,
        PullUp, PullDown, SlewRate
    };
    // use super::DriveStrength;
    use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin, InputPin};
    use riscv::interrupt;

    const GPIO_PTR: *const pac::gpioa::RegisterBlock = pac::GPIOA::ptr();

    const PORT_PTR: *const pac::porta::RegisterBlock = pac::PORTA::ptr();

    impl GpioExt for (pac::GPIOA, pac::PORTA) {
        type Parts = Parts;

        fn split(self) -> Self::Parts {
            Parts { 
                pta24: PTA24 { _mode: PhantomData, } 
            }
        }
    }

    pub struct Parts {
        pub pta24: PTA24<Floating>,
    }

    const PIN_INDEX: usize = 24;

    const PIN_MASK: u32 = 1 << PIN_INDEX;

    pub struct PTA24<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl<MODE> PTA24<MODE> {
        pub fn into_push_pull_output(self) -> PTA24<Output<PushPull>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ode().clear_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() | PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        pub fn into_open_drain_output(self) -> PTA24<Output<OpenDrain>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ode().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() | PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        pub fn into_floating_input(self) -> PTA24<Input<Floating>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.pe().clear_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        pub fn into_pull_up_input(self) -> PTA24<Input<PullUp>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ps().set_bit().pe().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        pub fn into_pull_down_input(self) -> PTA24<Input<PullDown>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ps().clear_bit().pe().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }
    }

    impl<MODE> PTA24<Output<MODE>> {
        pub fn set_slew_rate(&self, value: SlewRate) {
            unsafe { &*PORT_PTR }.pcr24.write(|w| match value {
                SlewRate::Fast => w.sre().clear_bit(),
                SlewRate::Slow => w.sre().set_bit(),
            });
        }
    }

    // // not all pins support drive strength config
    // impl<MODE> PTA24<Output<MODE>> {
    //     pub fn set_drive_strength(&self, value: DriveStrength) {
    //         unsafe { &*PORT_PTR }.pcr24.write(|w| match value {
    //             DriveStrength::Low => w.dse().clear_bit(),
    //             DriveStrength::High => w.dse().set_bit(),
    //         });
    //     }
    // }

    impl<MODE> PTA24<MODE> {
        pub fn lock(self) -> Locked<PTA24<MODE>> {
            unsafe { &*PORT_PTR }.pcr0.write(|w| w.lk().set_bit());
            Locked(self)
        }
    }

    impl<MODE> OutputPin for PTA24<Output<MODE>> {
        type Error = Infallible;

        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.pcor.write(|w| unsafe { w.ptco().bits(PIN_MASK) });
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.psor.write(|w| unsafe { w.ptso().bits(PIN_MASK) });
            Ok(())
        }
    }

    impl<MODE> StatefulOutputPin for PTA24<Output<MODE>> {
        fn is_set_high(&self) -> Result<bool, Infallible> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & PIN_MASK != 0)
        }

        fn is_set_low(&self) -> Result<bool, Infallible> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & PIN_MASK == 0)
        }
    }

    impl<MODE> ToggleableOutputPin for PTA24<Output<MODE>> {
        type Error = Infallible;

        fn toggle(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.ptor.write(|w| unsafe { w.ptto().bits(PIN_MASK) });
            Ok(())
        }
    }

    impl<MODE> InputPin for PTA24<Input<MODE>> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & PIN_MASK != 0) 
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & PIN_MASK == 0) 
        }
    }

    impl InputPin for PTA24<Output<OpenDrain>> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & PIN_MASK != 0) 
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & PIN_MASK == 0) 
        }
    }
}

// todo: port[4]: pfe (passive filter enable)
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
