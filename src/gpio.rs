use crate::pac;
use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait GpioExt {
    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/*
    This chip supports BATCH modifications of port settings;
    hal designs should take best use of this feature.
    (luojia65)
*/

/// General-purpose input, for the GPIO function
pub struct Input<MODE> {
    _typestate_mode: PhantomData<MODE>,
}

/// General-purpose output, for the GPIO function
pub struct Output<MODE> {
    _typestate_mode: PhantomData<MODE>,
}

// PCRx::DSE, Drive Strength Enable
// 0b&&output->low
// 1b&&output->high
// todo

// PCRx::ODE, Open Drain Enable
// 0b->disable, input->disable
// 1b&&output->enable
pub struct OpenDrain;

// PCRx::SRE Slew Rate Enable
// 0b&&output->fast
// 1b&&output->slow

// PCRx::PE Pull Enable = 1
// PCRx::PS Pull Select = 1 (up), input
pub struct PullUp;

// PCRx::PE Pull Enable = 1
// PCRx::PS Pull Select = 0 (down), input
pub struct PullDown;

// default Hi-Z state
pub struct Floating;

// this chip supports pin locking

pub struct Locked;

pub struct Unlocked;

trait PeripheralAccess {
    fn peripheral() -> &'static pac::gpioa::RegisterBlock;

    /// Set port data direction to input with PDDR register
    fn set_dir_input(index: usize) {
        let p = Self::peripheral();
        let mask = (1 << (index & 31)) as u32;
        p.pddr.modify(|r, w| unsafe {
            w.pdd().bits(r.pdd().bits() & !mask)
        });
    }

    /// Set port data direction to output with PDDR register
    fn set_dir_output(index: usize) {
        let p = Self::peripheral();
        let mask = (1 << (index & 31)) as u32;
        p.pddr.modify(|r, w| unsafe {
            w.pdd().bits(r.pdd().bits() | mask)
        });
    }

    /// Read port data with PDIR register
    fn value(index: usize) -> bool {
        let p = Self::peripheral();
        (p.pdir.read().bits() >> ((index & 31) as u32)) != 0
    }

    /// Set output with PSOR/PCOR register
    fn set_output(index: usize, bit: bool) {
        let p = Self::peripheral();
        let mask = (1 << (index & 31)) as u32;
        if bit {
            p.psor.write(|w| unsafe { w.ptso().bits(mask) });
        } else {
            p.pcor.write(|w| unsafe { w.ptco().bits(mask) });
        }
    }

    /// Toggle output with PTOR register
    fn toggle_output(index: usize) {
        let p = Self::peripheral();
        let mask = (1 << (index & 31)) as u32;
        p.ptor.write(|w| unsafe { w.ptto().bits(mask) });
    }
}

pub mod gpioa {
    use crate::pac;
    use core::convert::Infallible;
    use core::marker::PhantomData;
    use embedded_hal::digital::v2::{StatefulOutputPin, OutputPin, ToggleableOutputPin};
    use super::{GpioExt, PeripheralAccess, Unlocked, Locked, Floating};
    // use crate::pac::flexio0::PIN;

    pub struct Parts {
        pub pta24: PTA24<Unlocked, Floating>,
    }

    impl PeripheralAccess for pac::GPIOA {
        #[inline(always)]
        fn peripheral() -> &'static pac::gpioa::RegisterBlock {
            unsafe { &*pac::GPIOA::ptr() }
        }
    }

    const PIN_INDEX: usize = 24;

    impl GpioExt for pac::GPIOA {
        type Parts = Parts;

        fn split(self) -> Self::Parts {
            pac::GPIOA::set_dir_output(PIN_INDEX);
            Parts { 
                pta24: PTA24 { 
                    _typestate_locked: PhantomData,
                    _typestate_mode: PhantomData,
                } 
            }
        }
    }

    pub struct PTA24<LOCKED, MODE> {
        _typestate_locked: PhantomData<LOCKED>,
        _typestate_mode: PhantomData<MODE>,
    }

    impl<LOCKED, MODE> PTA24<LOCKED, MODE> {
        // pub fn into_open_drain_output() ...
    }

    impl<MODE> PTA24<Unlocked, MODE> {
        pub fn lock(self) -> PTA24<Locked, MODE> {
            unimplemented!() // todo
        }
    }

    impl<LOCKED, MODE> StatefulOutputPin for PTA24<LOCKED, MODE> {
        fn is_set_high(&self) -> Result<bool, Infallible> {
            Ok(pac::GPIOA::value(PIN_INDEX))
        }

        fn is_set_low(&self) -> Result<bool, Infallible> {
            Ok(!pac::GPIOA::value(PIN_INDEX))
        }
    }

    impl<LOCKED, MODE> OutputPin for PTA24<LOCKED, MODE> {
        type Error = Infallible;

        fn set_low(&mut self) -> Result<(), Self::Error> {
            pac::GPIOA::set_output(PIN_INDEX, false);
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            pac::GPIOA::set_output(PIN_INDEX, true);
            Ok(())
        }
    }

    impl<LOCKED, MODE> ToggleableOutputPin for PTA24<LOCKED, MODE> {
        type Error = Infallible;

        fn toggle(&mut self) -> Result<(), Self::Error> {
            pac::GPIOA::toggle_output(PIN_INDEX);
            Ok(())
        }
    }
}
