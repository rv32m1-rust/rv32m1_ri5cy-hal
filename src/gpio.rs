use rv32m1_ri5cy_pac as pac;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait GpioExt {
    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// General-purpose input, for the GPIO function
pub struct Input;

/// General-purpose output, for the GPIO function
pub struct Output;

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
    use rv32m1_ri5cy_pac as pac;
    use core::convert::Infallible;
    use embedded_hal::digital::v2::{StatefulOutputPin, OutputPin, ToggleableOutputPin};
    use super::{GpioExt, PeripheralAccess};
    use core::any::Any;
    use rv32m1_ri5cy_pac::flexio0::PIN;

    pub struct Parts {
        pub p24: Pin24,
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
            Parts { p24: Pin24 {} }
        }
    }

    pub struct Pin24;

    impl StatefulOutputPin for Pin24 {
        fn is_set_high(&self) -> Result<bool, Infallible> {
            Ok(pac::GPIOA::value(PIN_INDEX))
        }

        fn is_set_low(&self) -> Result<bool, Infallible> {
            Ok(!self.is_set_high()?)
        }
    }

    impl OutputPin for Pin24 {
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

    impl ToggleableOutputPin for Pin24 {
        type Error = Infallible;

        fn toggle(&mut self) -> Result<(), Self::Error> {
            pac::GPIOA::toggle_output(PIN_INDEX);
            Ok(())
        }
    }
}
