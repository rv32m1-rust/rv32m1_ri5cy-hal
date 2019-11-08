//! HAL for the RI5CY core of RV32M1 SoC
//!
//! This is an implementation of the [`embedded-hal`] traits for the RI5CY
//! core of RV32M1 SoC.

#![no_std]

pub use rv32m1_ri5cy_pac as pac;
pub mod prelude;
pub mod gpio;
