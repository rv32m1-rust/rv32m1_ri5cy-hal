//! General-Purpose Input/Output (GPIO)

use core::marker::PhantomData;

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
    fn split(self, pcc_port: &mut Self::Clock) -> Result<Self::Parts, SplitError>;
}

/// Error that may occur when spliting port
#[derive(Clone, Copy, Debug)]
pub enum SplitError {
    /// The port peripheral is not present on this device.
    Absent,
    /// Peripheral is being used by another core.
    InUse,
}

/// Pin slew rate, valid in all digital pin muxing modes.
#[derive(Clone, Copy, Debug)]
pub enum SlewRate {
    /// Fast slew rate is configured on the corresponding pin, 
    /// if the pin is configured as a digital output.
    Fast,
    /// Slow slew rate is configured on the corresponding pin, 
    /// if the pin is configured as a digital output.
    Slow
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

// macro_rules! gpio {
//     ($gpiox: ident) => {    
/// GPIO
pub mod gpioa {
    use crate::{pac, pcc};
    use core::{convert::Infallible, marker::PhantomData};
    use super::{
        GpioExt, SplitError, Locked, Output, OpenDrain, PushPull, Input, Floating,
        PullUp, PullDown, SlewRate
    };
    // use super::DriveStrength;
    use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin, InputPin};
    use riscv::interrupt;

    const GPIO_PTR: *const pac::gpioa::RegisterBlock = pac::GPIOA::ptr();

    const PORT_PTR: *const pac::porta::RegisterBlock = pac::PORTA::ptr();

    impl GpioExt for (pac::GPIOA, pac::PORTA) {
        type Clock = pcc::PORTA;

        type Parts = Parts;

        fn split(self, pcc_port: &mut pcc::PORTA) -> Result<Self::Parts, SplitError> {
            interrupt::free(|_| {
                // if the port is not absent on this device, throw an error
                if !pcc_port.port().read().pr().is_pr_1() {
                    return Err(SplitError::Absent)
                }
                // if the port is being used by another core, throw an error.
                // That means, software on another core has already configured the clocking 
                // options of this peripheral.
                // For example, if your CPU software is the first to occupy this peripheral,
                // the INUSE bit would return 0, and settings to CGC bit would success. But
                // if it's not your CPU first to occupy, the INUSE bit would return 1 meaning
                // that this port is somehow locked by other CPUs.
                if pcc_port.port().read().inuse().is_inuse_1() {
                    return Err(SplitError::InUse)
                }
                // enable port clock
                pcc_port.port().write(|w| w.cgc().set_bit());
                Ok(Parts { 
                    pta24: PTA24 { _mode: PhantomData }, 
                })
            })
        } 
    }

    /// GPIO parts
    pub struct Parts {
        /// Pin
        pub pta24: PTA24<Input<Floating>>,
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
        pub fn free(self, pcc_port: &mut pcc::PORTA) -> (pac::GPIOA, pac::PORTA) {
            use core::mem::transmute;
            // release port clock
            pcc_port.port().write(|w| w.cgc().clear_bit());
            // return the ownership of GPIOA and PORTA
            unsafe { (transmute(()), transmute(())) }
        }
    }

    /// Partially erased pin
    pub struct PTAx<MODE> {
        i: u8,
        _mode: PhantomData<MODE>,
    }

    impl<MODE> PTAx<MODE> {
        #[inline]
        fn pin_mask(&self) -> u32 {
            1 << self.i
        }
    }

    impl<MODE> OutputPin for PTAx<Output<MODE>> {
        type Error = Infallible;
        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.pcor.write(|w| unsafe { w.ptco().bits(self.pin_mask()) });
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.psor.write(|w| unsafe { w.ptso().bits(self.pin_mask()) });
            Ok(())
        }
    }
    
    impl<MODE> InputPin for PTAx<Input<MODE>> {
        type Error = Infallible;
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() != 0) 
        }
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() == 0) 
        }
    }

    impl<MODE> StatefulOutputPin for PTAx<Output<MODE>> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & self.pin_mask() != 0)
        }
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & self.pin_mask() == 0)
        }
    }

    impl<MODE> ToggleableOutputPin for PTAx<Output<MODE>> {
        type Error = Infallible;
        fn toggle(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.ptor.write(|w| unsafe { w.ptto().bits(self.pin_mask()) });
            Ok(())
        }
    }

    impl InputPin for PTAx<Output<OpenDrain>> {
        type Error = Infallible;
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() != 0) 
        }
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & self.pin_mask() == 0) 
        }
    }

    const PIN_INDEX: u8 = 24;

    const PIN_MASK: u32 = 1 << PIN_INDEX;

    /// Pin
    pub struct PTA24<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl<MODE> PTA24<MODE> {
        /// Configures the pin to operate as a push-pull output pin.
        pub fn into_push_pull_output(self) -> PTA24<Output<PushPull>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ode().clear_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() | PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        /// Configures the pin to operate as an open-drain output pin.
        pub fn into_open_drain_output(self) -> PTA24<Output<OpenDrain>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ode().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() | PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        /// Configures the pin to operate as a floating input pin.
        pub fn into_floating_input(self) -> PTA24<Input<Floating>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.pe().clear_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        /// Configures the pin to operate as a pull-up input pin.
        pub fn into_pull_up_input(self) -> PTA24<Input<PullUp>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ps().set_bit().pe().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }

        /// Configures the pin to operate as a pull-down input pin.
        pub fn into_pull_down_input(self) -> PTA24<Input<PullDown>> {
            interrupt::free(|_| unsafe {
                (&*PORT_PTR).pcr24.write(|w| w.ps().clear_bit().pe().set_bit());
                (&*GPIO_PTR).pddr.modify(|r, w| w.pdd().bits(r.pdd().bits() & !PIN_MASK));
            });
            PTA24 { _mode: PhantomData } 
        }
    }

    impl<MODE> PTA24<Output<MODE>> {
        /// Change the pin's slew rate.
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
        /// Erases the pin number from the type
        ///
        /// This is useful when you want to collect the pins into an array where you
        /// need all the elements to have the same type
        pub fn downgrade(self) -> PTAx<MODE> {
            PTAx {
                i: PIN_INDEX,
                _mode: PhantomData,
            }
        }
    }

    impl<MODE> PTA24<MODE> {
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
//     };
// }

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
// gpio! { gpioa }
