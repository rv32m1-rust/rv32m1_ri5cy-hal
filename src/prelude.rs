//! Prelude 

pub use embedded_hal::prelude::*;
pub use embedded_hal::digital::v2::{
    InputPin as _embedded_hal_digital_v2_InputPin, 
    OutputPin as _embedded_hal_digital_v2_OutputPin, 
    StatefulOutputPin as _embedded_hal_digital_v2_StatefulOutputPin,
    ToggleableOutputPin as _embedded_hal_digital_v2_ToggleableOutputPin,
};
pub use crate::gpio::GpioExt as _rv32m1_ri5cy_hal_gpio_GpioExt;
pub use crate::pcc::PccExt as _rv32m1_ri5cy_hal_pcc_PccExt;
