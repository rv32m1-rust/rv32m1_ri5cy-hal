//! Time Stamp Timer Module (TSTMR)
//! 
//! The Time Stamp Timer Module is a 56-bit clock cycle counter, reset by system reset.
//! 
//! Each set of TSTMR registers can be accessed from the processor it's associated with.
//! All processor A's may access only TSTMRA.
//! 
//! Ref: Reference Manual, Chapter 52.2

use crate::pac::TSTMRA;

/// Get the 56-bit timestamp value from TSTMRA registers
/// 
/// The high 8 bits would always be zero.
pub fn timestamp() -> u64 {
    // note(unsafe): reference from a read-only register
    let tstmra = unsafe { &*TSTMRA::ptr() };
    let high = tstmra.high.read().bits();
    let low = tstmra.low.read().bits();
    ((high as u64) << 32) | (low as u64)
}
