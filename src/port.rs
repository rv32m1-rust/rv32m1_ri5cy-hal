pub trait PortExt {
    type Clock;

    type Parts;

    fn split(self, clock: &mut Self::Clock) -> Result<Self::Parts, SplitError>;
}

#[derive(Clone, Copy, Debug)]
pub enum SplitError {
    Absent,
    InUse,
}

pub struct Pin<T, M, LK>(T, M, LK);

pub struct Unknown;

pub struct PushPull;

pub struct OpenDrain;

pub struct Floating;

pub struct PullUp;

pub struct PullDown;

pub struct ALT0;

pub struct ALT1;

pub struct ALT2;

pub struct ALT3;

pub struct ALT4;

pub struct ALT5;

pub struct ALT6;

pub struct ALT7;

pub struct Locked;

pub struct Unlocked;

pub(crate) trait PinIndex {
    const INDEX: u32;

    const MASK: u32 = 1 << Self::INDEX;
}

pub mod porta {
    use crate::{pac, pcc};
    use riscv::interrupt;
    use super::{
        PortExt, SplitError, Pin, Unknown, Locked, Unlocked,
        PushPull, OpenDrain, Floating, PullUp, PullDown,
        ALT0, ALT1, ALT2, ALT3, ALT4, ALT5, ALT6, ALT7
    };

    const PORT_PTR: *const pac::porta::RegisterBlock = pac::PORTA::ptr();

    pub struct Parts {
        pub pta24: Pin<PTA24, (Unknown, Unknown, Unknown), Unknown>,
    }

    impl PortExt for pac::PORTA {
        type Clock = pcc::PORTA;

        type Parts = Parts;

        fn split(self, clock: &mut Self::Clock) -> Result<Self::Parts, SplitError> {
            interrupt::free(|_| {
                if !clock.reg().read().pr().is_pr_1() {
                    return Err(SplitError::Absent)
                }
                if clock.reg().read().inuse().is_inuse_1() {
                    return Err(SplitError::InUse)
                }
                clock.reg().write(|w| w.cgc().set_bit());
                Ok(Parts {
                    pta24: Pin(PTA24 { _ownership: () }, (Unknown, Unknown, Unknown), Unknown)
                })
            })
        }
    }

    impl Parts {
        pub fn free(self, clock: &mut pcc::PORTA) -> pac::PORTA {
            clock.reg().write(|w| w.cgc().clear_bit());
            unsafe { core::mem::transmute(()) }
        }
    }

    pub struct PTA24 {
        _ownership: ()
    }

    impl super::PinIndex for PTA24 {
        const INDEX: u32 = 24;
    }

    impl<MODE, LK> Pin<PTA24, MODE, LK> { 
        #[inline]
        pub fn unlocked(self) -> Pin<PTA24, MODE, Unlocked> {
            let is_locked = unsafe { &*PORT_PTR }.pcr24.read().lk().bit_is_set();
            if is_locked {
                panic!("pin is locked")
            }
            unsafe { self.assume_unlocked() }
        }

        #[inline]
        pub unsafe fn assume_unlocked(self) -> Pin<PTA24, MODE, Unlocked> {
            Pin(self.0, self.1, Unlocked)
        }
    }

    impl<MODE> Pin<PTA24, MODE, Unlocked> {
        #[inline]
        pub fn lock(self) -> Pin<PTA24, MODE, Locked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.lk().set_bit());
            Pin(self.0, self.1, Locked)
        }
    }

    impl<IM, OM, ALT> Pin<PTA24, (IM, OM, ALT), Unlocked> {
        #[inline]
        pub fn set_push_pull(self) -> Pin<PTA24, (IM, PushPull, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ode().clear_bit());
            Pin(self.0, ((self.1).0, PushPull, (self.1).2), Unlocked)
        }

        #[inline]
        pub fn set_open_drain(self) -> Pin<PTA24, (IM, OpenDrain, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ode().set_bit());
            Pin(self.0, ((self.1).0, OpenDrain, (self.1).2), Unlocked)
        }

        #[inline]
        pub fn set_floating(self) -> Pin<PTA24, (Floating, OM, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.pe().clear_bit());
            Pin(self.0, (Floating, (self.1).1, (self.1).2), Unlocked)
        }

        #[inline]
        pub fn set_pull_up(self) -> Pin<PTA24, (PullUp, OM, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ps().set_bit().pe().set_bit());
            Pin(self.0, (PullUp, (self.1).1, (self.1).2), Unlocked)
        }

        #[inline]
        pub fn set_pull_down(self) -> Pin<PTA24, (PullDown, OM, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ps().clear_bit().pe().set_bit());
            Pin(self.0, (PullDown, (self.1).1, (self.1).2), Unlocked)
        }

        #[inline]
        pub fn into_alt0(self) -> Pin<PTA24, (IM, OM, ALT0), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT0), Unlocked)
        }

        #[inline]
        pub fn into_alt1(self) -> Pin<PTA24, (IM, OM, ALT1), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT1), Unlocked)
        }

        #[inline]
        pub fn into_alt2(self) -> Pin<PTA24, (IM, OM, ALT2), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT2), Unlocked)
        }

        #[inline]
        pub fn into_alt3(self) -> Pin<PTA24, (IM, OM, ALT3), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT3), Unlocked)
        }

        #[inline]
        pub fn into_alt4(self) -> Pin<PTA24, (IM, OM, ALT4), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT4), Unlocked)
        }

        #[inline]
        pub fn into_alt5(self) -> Pin<PTA24, (IM, OM, ALT5), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT5), Unlocked)
        }

        #[inline]
        pub fn into_alt6(self) -> Pin<PTA24, (IM, OM, ALT6), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT6), Unlocked)
        }

        #[inline]
        pub fn into_alt7(self) -> Pin<PTA24, (IM, OM, ALT7), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            Pin(self.0, ((self.1).0, (self.1).1, ALT7), Unlocked)
        }
    }
}
