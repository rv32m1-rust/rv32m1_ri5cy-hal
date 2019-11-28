pub struct Input;

pub struct Output;

pub trait GpioExt {
    type Output;

    fn constrain(self) -> Self::Output;
}

pub mod convert {
    pub trait IntoPushPullOutput {
        type Target;
        type Gpio;
        fn into_push_pull_output(self, gpio: &mut Self::Gpio) -> Self::Target;
    }
    pub trait IntoOpenDrainOutput {
        type Target;
        type Gpio;
        fn into_open_drain_output(self, gpio: &mut Self::Gpio) -> Self::Target;
    }
    pub trait IntoPullUpInput {
        type Target;
        type Gpio;
        fn into_pull_up_input(self, gpio: &mut Self::Gpio) -> Self::Target;
    }
    pub trait IntoPullDownInput{
        type Target;
        type Gpio;
        fn into_pull_down_input(self, gpio: &mut Self::Gpio) -> Self::Target;
    }
    pub trait IntoFloatingInput {
        type Target;
        type Gpio;
        fn into_floating_input(self, gpio: &mut Self::Gpio) -> Self::Target;
    }
}

pub mod gpioa {
    use core::convert::Infallible;
    use crate::pac;
    use crate::port::{
        PushPull, OpenDrain, Floating, PullUp, PullDown,
        ALT1, Unlocked, Locked, porta::PTA24
    };
    use super::{GpioExt, Input, Output};
    use super::convert::*;
    use riscv::interrupt;    
    use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin, InputPin};

    const GPIO_PTR: *const pac::gpioa::RegisterBlock = pac::GPIOA::ptr();

    pub struct GPIOA {
        gpioa: pac::GPIOA
    }

    impl GpioExt for pac::GPIOA {
        type Output = GPIOA;
    
        #[inline]
        fn constrain(self) -> GPIOA {
            GPIOA { gpioa: self }
        }
    }

    impl GPIOA {
        #[inline]
        pub fn free(self) -> pac::GPIOA {
            self.gpioa
        }

        #[inline]
        unsafe fn pddr(&self) -> &pac::gpioa::PDDR {
            &(*pac::GPIOA::ptr()).pddr
        }
    }
    
    impl<IM, OM, DIR> PA24<IM, OM, Unlocked, DIR> { 
        #[inline]
        pub fn lock(self) -> PA24<IM, OM, Locked, DIR> {
            PA24(self.0.lock(), self.1)
        }
    }

    pub struct PA24<IM, OM, LK, DIR>(PTA24<(IM, OM, ALT1), LK>, DIR);

    impl<IM, OM, LK, DIR> PA24<IM, OM, LK, DIR> {
        #[inline]
        pub fn free(self) -> PTA24<(IM, OM, ALT1), LK> {
            self.0
        }
    }
    
    impl<IM, OM, DIR> IntoPushPullOutput for PA24<IM, OM, Unlocked, DIR> {
        type Target = PA24<IM, PushPull, Unlocked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_push_pull_output(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() | 1 << 24)) };
                self.0.set_push_pull().into_alt1()
            }), Output)
        }
    }

    impl<IM, DIR> IntoPushPullOutput for PA24<IM, PushPull, Locked, DIR> {
        type Target = PA24<IM, PushPull, Locked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_push_pull_output(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() | 1 << 24)) };
                self.0
            }), Output)
        }
    }

    impl<IM, OM, DIR> IntoOpenDrainOutput for PA24<IM, OM, Unlocked, DIR> {
        type Target = PA24<IM, OpenDrain, Unlocked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_open_drain_output(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() | 1 << 24)) };
                self.0.set_open_drain().into_alt1()
            }), Output)
        }
    }

    impl<IM, DIR> IntoOpenDrainOutput for PA24<IM, OpenDrain, Locked, DIR> {
        type Target = PA24<IM, OpenDrain, Locked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_open_drain_output(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() | 1 << 24)) };
                self.0
            }), Output)
        }
    }

    impl<IM, OM, DIR> IntoFloatingInput for PA24<IM, OM, Unlocked, DIR> {
        type Target = PA24<Floating, OM, Unlocked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_floating_input(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.0.set_floating().into_alt1()
            }), Output)
        }
    }

    impl<OM, DIR> IntoFloatingInput for PA24<Floating, OM, Locked, DIR> {
        type Target = PA24<Floating, OM, Locked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_floating_input(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.0
            }), Output)
        }
    }

    impl<IM, OM, DIR> IntoPullUpInput for PA24<IM, OM, Unlocked, DIR> {
        type Target = PA24<PullUp, OM, Unlocked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_pull_up_input(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.0.set_pull_up().into_alt1()
            }), Output)
        }
    }

    impl<OM, DIR> IntoPullUpInput for PA24<PullUp, OM, Locked, DIR> {
        type Target = PA24<PullUp, OM, Locked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_pull_up_input(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.0
            }), Output)
        }
    }

    impl<IM, OM, DIR> IntoPullDownInput for PA24<IM, OM, Unlocked, DIR> {
        type Target = PA24<PullDown, OM, Unlocked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_pull_down_input(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.0.set_pull_down().into_alt1()
            }), Output)
        }
    }

    impl<OM, DIR> IntoPullDownInput for PA24<PullDown, OM, Locked, DIR> {
        type Target = PA24<PullDown, OM, Locked, Output>;
        type Gpio = GPIOA;
        #[inline]
        fn into_pull_down_input(self, gpioa: &mut GPIOA) -> Self::Target {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.0
            }), Output)
        }
    }

    impl<IM, OM, ALT> PTA24<(IM, OM, ALT), Unlocked> {
        #[inline]
        pub fn into_push_pull_output(self, gpioa: &mut GPIOA) -> PA24<IM, PushPull, Unlocked, Output> {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() | 1 << 24)) };
                self.set_push_pull().into_alt1()
            }), Output)
        }
        
        #[inline]
        pub fn into_open_drain_output(self, gpioa: &mut GPIOA) -> PA24<IM, OpenDrain, Unlocked, Output> {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() | 1 << 24)) };
                self.set_open_drain().into_alt1()
            }), Output)
        }
        
        #[inline]
        pub fn into_floating_input(self, gpioa: &mut GPIOA) -> PA24<Floating, OM, Unlocked, Input> {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.set_floating().into_alt1()
            }), Input)
        }
        
        #[inline]
        pub fn into_pull_up_input(self, gpioa: &mut GPIOA) -> PA24<PullUp, OM, Unlocked, Input>  {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.set_pull_up().into_alt1()
            }), Input)
        }
        
        #[inline]
        pub fn into_pull_down_input(self, gpioa: &mut GPIOA) -> PA24<PullDown, OM, Unlocked, Input>  {
            PA24(interrupt::free(|_| {
                unsafe { gpioa.pddr().modify(|r, w| w.pdd().bits(r.pdd().bits() & !(1 << 24))) };
                self.set_pull_down().into_alt1()
            }), Input)
        }
    }

    impl<IM, OM, LK> OutputPin for PA24<IM, OM, LK, Output> {
        type Error = Infallible;
        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.pcor.write(|w| unsafe { w.ptco().bits(1 << 24) });
            Ok(())
        }
        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.psor.write(|w| unsafe { w.ptso().bits(1 << 24) });
            Ok(())
        }
    }

    impl<IM, LK> InputPin for PA24<IM, OpenDrain, LK, Output> {
        type Error = Infallible;
        #[inline]
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & 1 << 24 != 0)
        }
        #[inline]
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & 1 << 24 == 0)
        }
    }

    impl<IM, OM, LK> InputPin for PA24<IM, OM, LK, Input> {
        type Error = Infallible;
        #[inline]
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & 1 << 24 != 0)
        }
        #[inline]
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdir.read().bits() & 1 << 24 == 0)
        }
    }

    impl<IM, OM, LK> StatefulOutputPin for PA24<IM, OM, LK, Output>  {
        #[inline]
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & 1 << 24 != 0)
        }
        #[inline]
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(unsafe { &*GPIO_PTR }.pdor.read().bits() & 1 << 24 == 0)
        }
    }

    impl<IM, OM, LK> ToggleableOutputPin for PA24<IM, OM, LK, Output> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            unsafe { &*GPIO_PTR }.ptor.write(|w| unsafe { w.ptto().bits(1 << 24) });
            Ok(())
        }
    }
}
