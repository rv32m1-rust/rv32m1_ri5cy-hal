//! HAL for the RI5CY core of RV32M1 SoC
//!
//! This is an implementation of the [`embedded-hal`] traits for the RI5CY
//! core of RV32M1 SoC.

#![no_std]

pub use rv32m1_ri5cy_pac as pac;
pub mod gpio;
pub mod lptmr;
pub mod pcc;
pub mod serial;
pub mod tstmr;

pub mod prelude {
    //! Prelude

    pub use crate::gpio::GpioExt as _rv32m1_ri5cy_hal_gpio_GpioExt;
    pub use crate::pcc::PccExt as _rv32m1_ri5cy_hal_pcc_PccExt;
    pub use embedded_hal::digital::{
        InputPin as _embedded_hal_digital_InputPin, OutputPin as _embedded_hal_digital_OutputPin,
        StatefulOutputPin as _embedded_hal_digital_StatefulOutputPin,
        ToggleableOutputPin as _embedded_hal_digital_ToggleableOutputPin,
    };
    pub use embedded_hal::prelude::*;
}
