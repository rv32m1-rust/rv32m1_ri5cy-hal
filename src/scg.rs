//! System clock configurations
//!
//! This module constrains the System Clock Generator (SCG) peripheral.

use embedded_time::rate::Hertz;

/// Frozen clock freqencies
pub struct Clocks {}

impl Clocks {
    /// Returns the frequency of the System OSC clock
    pub fn sysosc(&self) -> Hertz {
        Hertz::new(72_000_000)
    }
    /// Returns the frequency of the Slow IRC clock
    pub fn sirc(&self) -> Hertz {
        Hertz::new(0)
    }
    /// Returns the frequency of the Fast IRC clock
    pub fn hirc(&self) -> Hertz {
        Hertz::new(0)
    }
    /// Returns the frequency of the Low Power FLL clock
    pub fn lpfll(&self) -> Hertz {
        Hertz::new(0)
    }
}

impl Clocks {
    pub(crate) fn of_source(&self, source: Source) -> Hertz {
        match source {
            Source::SysOsc => self.sysosc(),
            Source::Sirc => self.sirc(),
            Source::Hirc => self.hirc(),
            Source::LpFll => self.lpfll(),
        }
    }
}

/// Clock source
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Source {
    SysOsc,
    Sirc,
    Hirc,
    LpFll,
}

/// Precise clock configurator
pub struct Precise {}

/// Strict clock configurator
pub struct Strict {}
