//! Timer modules

use embedded_time::rate::*; 
use crate::pac::LPTMR0; //{LPTMR0, LPTMR1};
use core::convert::Infallible;

pub struct Timer<T> {
    tmr: T
}

impl Timer<LPTMR0> {
    /// Initialize the timer 
    pub fn lptmr0(tmr: LPTMR0, /* clocks: ..., pcc: ... */) -> Self {
        /* enable clocks ... */
        // disable the timer
        tmr.csr.modify(|_r, w| w.ten().ten_0());
        Self { tmr }
    }

    /// Start a new countdown timer
    pub fn start_count_down<T>(self, timeout: T) -> CountDown<LPTMR0> 
    where 
        T: Into<Hertz>
    {
        self.tmr.csr.modify(|_r, w| w
            .tms().tms_0() // timer counter mode
            .tfc().tfc_0() // disable free running
            .tpp().tpp_0() // active high
            .tps().tps_0() // input #0
        );
        self.tmr.psr.modify(|_r, w| w
            .prescale().prescale_0() // prescale = 1
            .pbyp().pbyp_1() // bypass prescaler
            .pcs().pcs_1() // prescaler clock 1
        );
        // todo: calculate ticks
        let hertz = timeout.into();

        let mut ans = CountDown { tmr: self.tmr, ticks: hertz.0 };
        use embedded_hal::timer::CountDown as _embedded_hal_timer_CountDown;
        ans.try_start(hertz).ok();
        ans
    }

    /// Releases the count down timer
    pub fn release(self) -> LPTMR0 {
        // return ownership
        self.tmr
    }
}

/// Timer update events
pub enum Event {
    /// Timer timed out / count down ended
    Update,
}

pub struct CountDown<T> {
    ticks: u32,
    tmr: T,
}

impl CountDown<LPTMR0> {
    /// Enable LPTMR interrupt
    pub fn listen(&mut self, event: Event) {
        drop(event); // note(drop): only one case
        self.tmr.csr.modify(|_r, w| w.tie().tie_1());
    }

    /// Disable LPTMR interrupt
    pub fn unlisten(&mut self, event: Event) {
        drop(event); // note(drop): only one case
        self.tmr.csr.modify(|_r, w| w.tie().tie_0());
    }

    /// Stop the count down timer
    pub fn stop(self) -> Timer<LPTMR0> {
        // disable the timer
        self.tmr.csr.modify(|_r, w| w.ten().ten_0());
        // return ownership
        Timer { tmr: self.tmr }
    }

    /// Returns the number of ticks since the last update event.
    pub fn ticks_since(&self) -> u32 {
        self.tmr.cnr.read().bits()
    }

    /// Releases the count down timer
    pub fn release(self) -> LPTMR0 {
        self.stop().release()
    }
}

impl embedded_hal::timer::CountDown for CountDown<LPTMR0> {
    /// An enumeration of `CountDown` errors.
    ///
    /// For infallible implementations, will be `Infallible`
    type Error = Infallible;

    /// The unit of time used by this timer
    type Time = Hertz;

    /// Starts a new count down
    fn try_start<T>(&mut self, count: T) -> Result<(), Self::Error>
    where
        T: Into<Self::Time>
    {
        self.ticks = count.into().0;
        // clean flag
        self.tmr.csr.modify(|_r, w| w
            .tcf().set_bit() // W1C; clean the flag
        );
        // todo: refresh ticks
        // note(unsafe): ensured proper tick value
        self.tmr.cmr.modify(|_r, w| unsafe {
            w.compare().bits(self.ticks)
        });
        // enable the timer
        self.tmr.csr.modify(|_r, w| w.ten().ten_1());
        Ok(())
    }

    /// Non-blockingly "waits" until the count down finishes
    ///
    /// # Contract
    ///
    /// - If `Self: Periodic`, the timer will start a new count down right after the last one
    /// finishes.
    /// - Otherwise the behavior of calling `try_wait` after the last call returned `Ok` is UNSPECIFIED.
    /// Implementers are suggested to panic on this scenario to signal a programmer error.
    fn try_wait(&mut self) -> nb::Result<(), Self::Error> {
        if self.tmr.csr.read().tcf().is_tcf_1() {
            Ok(())
            // todo: start new timer?
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal::timer::Periodic for CountDown<LPTMR0> {}
