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

pub mod porta {
    use crate::{pac, pcc};
    use riscv::interrupt;
    use super::{
        PortExt, SplitError, Unknown, Locked, Unlocked,
        PushPull, OpenDrain, Floating, PullUp, PullDown,
        ALT0, ALT1, ALT2, ALT3, ALT4, ALT5, ALT6, ALT7
    };

    const PORT_PTR: *const pac::porta::RegisterBlock = pac::PORTA::ptr();

    pub struct Parts {
        pub pta24: PTA24<(Unknown, Unknown, Unknown), Unknown>,
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
                    pta24: PTA24((Unknown, Unknown, Unknown), Unknown),
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

    pub struct PTA24<MODE, LK>(MODE, LK);
    
    impl<MODE> PTA24<MODE, Unlocked> {
        #[inline]
        pub fn lock(self) -> PTA24<MODE, Locked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.lk().set_bit());
            PTA24(self.0, Locked)
        }
    }

    impl<MODE> PTA24<MODE, Unknown> { 
        #[inline]
        pub fn unlocked(self) -> PTA24<MODE, Unlocked> {
            let is_locked = unsafe { &*PORT_PTR }.pcr24.read().lk().bit_is_set();
            if is_locked {
                panic!("pin is locked")
            }
            unsafe { self.assume_unlocked() }
        }

        #[inline]
        pub unsafe fn assume_unlocked(self) -> PTA24<MODE, Unlocked> {
            PTA24(self.0, Unlocked)
        }
    }

    impl<MODE, LK> PTA24<MODE, LK> {
        #[inline]
        pub fn unknown(self) -> PTA24<(Unknown, Unknown, Unknown), Unknown> {
            PTA24((Unknown, Unknown, Unknown), Unknown)
        }
    }

    impl<IM, OM, ALT> PTA24<(IM, OM, ALT), Unlocked> {
        #[inline]
        pub fn set_push_pull(self) -> PTA24<(IM, PushPull, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ode().clear_bit());
            PTA24(((self.0).0, PushPull, (self.0).2), Unlocked)
        }

        #[inline]
        pub fn set_open_drain(self) -> PTA24<(IM, OpenDrain, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ode().set_bit());
            PTA24(((self.0).0, OpenDrain, (self.0).2), Unlocked)
        }

        #[inline]
        pub fn set_floating(self) -> PTA24<(Floating, OM, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.pe().clear_bit());
            PTA24((Floating, (self.0).1, (self.0).2), Unlocked)
        }

        #[inline]
        pub fn set_pull_up(self) -> PTA24<(PullUp, OM, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ps().set_bit().pe().set_bit());
            PTA24((PullUp, (self.0).1, (self.0).2), Unlocked)
        }

        #[inline]
        pub fn set_pull_down(self) -> PTA24<(PullDown, OM, ALT), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.ps().clear_bit().pe().set_bit());
            PTA24((PullDown, (self.0).1, (self.0).2), Unlocked)
        }

        #[inline]
        pub fn into_alt0(self) -> PTA24<(IM, OM, ALT0), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT0), Unlocked)
        }

        #[inline]
        pub fn into_alt1(self) -> PTA24<(IM, OM, ALT1), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT1), Unlocked)
        }

        #[inline]
        pub fn into_alt2(self) -> PTA24<(IM, OM, ALT2), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT2), Unlocked)
        }

        #[inline]
        pub fn into_alt3(self) -> PTA24<(IM, OM, ALT3), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT3), Unlocked)
        }

        #[inline]
        pub fn into_alt4(self) -> PTA24<(IM, OM, ALT4), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT4), Unlocked)
        }

        #[inline]
        pub fn into_alt5(self) -> PTA24<(IM, OM, ALT5), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT5), Unlocked)
        }

        #[inline]
        pub fn into_alt6(self) -> PTA24<(IM, OM, ALT6), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT6), Unlocked)
        }

        #[inline]
        pub fn into_alt7(self) -> PTA24<(IM, OM, ALT7), Unlocked> {
            unsafe { &*PORT_PTR }.pcr24.modify(|_, w| w.mux().mux_0());
            PTA24(((self.0).0, (self.0).1, ALT7), Unlocked)
        }
    }
}
