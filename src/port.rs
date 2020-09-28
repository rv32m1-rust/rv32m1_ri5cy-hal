//! Port peripheral

use crate::pcc::EnableError;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait PortExt {
    /// The port clock controller that controls this port
    type Clock;

    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    ///
    /// It's possible to have errors because this GPIO peripheral may be in use
    /// by another core, or the peripheral is absent on this device.
    fn split(self, pcc_port: &mut Self::Clock) -> Result<Self::Parts, EnableError>;
}

/// Alternate function 0 (type state)
pub struct ALT0;

/// Alternate function 1 (type state)
pub struct ALT1;

/// Alternate function 2 (type state)
pub struct ALT2;

/// Alternate function 3 (type state)
pub struct ALT3;

/// Alternate function 4 (type state)
pub struct ALT4;

/// Alternate function 5 (type state)
pub struct ALT5;

/// Alternate function 6 (type state)
pub struct ALT6;

/// Alternate function 7 (type state)
pub struct ALT7;

macro_rules! port_impl {
    ($PORTX: ident, $portx: ident, $PTXx: ident, [
        $($PTXi: ident:
            ($ptxi: ident, $i: expr, $PCRi: ident, $mode: ty),
            ($doc_name: expr, $pinout: expr),
            ($af0: tt, $af1: tt, $af2: tt, $af3: tt, $af4: tt, $af5: tt, $af6: tt, $af7: tt),
        )+
    ]) => {
/// Port
pub mod $portx {
    use super::{PortExt, ALT0, ALT1, ALT2, ALT3, ALT4, ALT5, ALT6, ALT7};
    use core::marker::PhantomData;
    use core::mem::MaybeUninit;
    use crate::{pac, pcc};
    use crate::pcc::EnableError;

    impl PortExt for pac::$PORTX {
        type Clock = pcc::$PORTX;

        type Parts = Parts;

        fn split(self, pcc_port: &mut pcc::$PORTX) -> Result<Self::Parts, EnableError> {
            pcc_port.try_enable()?;
            Ok(Parts {
                $(
                    $ptxi: $PTXi { pcr: unsafe { MaybeUninit::uninit().assume_init() }, _function: PhantomData },
                )+
            })
        }
    }

    impl Parts {
        /// Free and release the port taken so it could be used by another core.
        ///
        /// Users may use this function like:
        /// ```
        /// let pta0 = gpioa.pta0.into_push_pull_output();
        /// let pta24 = gpioa.pta24.into_push_pull_output();
        /// // code that uses pin pta0 and pta24 etc.
        /// // switch the port mode back before free operation.
        /// Parts {
        ///     pta0: pta0.into_floating_input(),
        ///     pta24: pta24.into_floating_input(),
        /// ..parts }.free(); // free this port
        /// ```
        pub fn free(self, pcc_port: &mut pcc::$PORTX) -> pac::$PORTX {
            use core::mem::transmute;
            // disable peripheral clock
            pcc_port.disable();
            // return the ownership of $PORTX
            unsafe { transmute(()) }
        }
    }

    /// Port parts
    pub struct Parts {
        $( pub $ptxi: $PTXi<$mode>, )+
    }

$(
    #[doc = "Pin"]
    #[doc = $doc_name]
    #[doc = "at"]
    #[doc = $pinout]
    pub struct $PTXi<AF> {
        pcr: pac::$portx::$PCRi,
        _function: PhantomData<AF>,
    }

    impl<AF> $PTXi<AF> {
        port_impl!(@af0 $PTXi, $af0);
        port_impl!(@afi $PTXi, "1", into_af1, ALT1, mux_1, $af1);
        port_impl!(@afi $PTXi, "2", into_af2, ALT2, mux_2, $af2);
        port_impl!(@afi $PTXi, "3", into_af3, ALT3, mux_3, $af3);
        port_impl!(@afi $PTXi, "4", into_af4, ALT4, mux_4, $af4);
        port_impl!(@afi $PTXi, "5", into_af5, ALT5, mux_5, $af5);
        port_impl!(@afi $PTXi, "6", into_af6, ALT6, mux_6, $af6);
        port_impl!(@afi $PTXi, "7", into_af7, ALT7, mux_7, $af7);
    }

    impl<AF> $PTXi<AF> {
        #[inline] pub(crate) fn into_af1_no_open_drain(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .ode().clear_bit() // Open Drain Enable: 0
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_with_open_drain(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .ode().set_bit() // Open Drain Enable: 1
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_no_pull(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().clear_bit() // Pull Enable: 0
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_pull_up(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().set_bit() // Pull Enable: 1
                .ps().set_bit() // Pull Select: 1 (pullup)
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
        #[inline] pub(crate) fn into_af1_pull_down(self) -> $PTXi<ALT1> {
            self.pcr.write(|w| 
                w.mux().mux_1() // Pin Mux Control: ALT1
                .pe().set_bit() // Pull Enable: 1
                .ps().clear_bit() // Pull Select: 0 (pulldown)
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
    }
)+
}
    };
    (@af0 $PTXi: ident, x) => {
        #[doc = "Configures the pin to operate as disabled (alternate function 0)"]
        pub fn into_disabled(self) -> $PTXi<ALT0> {
            self.pcr.write(|w| 
                w.mux().mux_0() // Pin Mux Control: ALT0
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
    };
    (@af0 $PTXi: ident, $doc: expr) => {
        #[doc = "Configures the pin to operate as alternate function 0 :"]
        #[doc = $doc]
        pub fn into_af0(self) -> $PTXi<ALT0> {
            self.pcr.write(|w| 
                w.mux().mux_0() // Pin Mux Control: ALT0
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
    };
    (@afi $PTXi: ident, $afi: expr, $into_afi: ident, $ALTi: ty, $mux_i: ident, x) => {}; // generate nothing
    (@afi $PTXi: ident, $afi: expr, $into_afi: ident, $ALTi: ty, $mux_i: ident, $doc: expr) => {
        #[doc = "Configures the pin to operate as alternate function"]
        #[doc = $afi]
        #[doc = ":"]
        #[doc = $doc]
        pub fn $into_afi(self) -> $PTXi<$ALTi> {
            self.pcr.write(|w| 
                w.mux().$mux_i() // Pin Mux Control: ALTi
            );
            $PTXi { pcr: self.pcr, _function: PhantomData }
        }
    };
}

port_impl! { PORTA, porta, PTAx, [    
    PTA0: (pta0, 0, PCR0, ALT7), ("PTA0", "B10"), (x, "PTA0", x, x, x, x, x, "NMI_b"),
    PTA1: (pta1, 1, PCR1, ALT7), ("PTA1/LLWU_P0", "E12"), (x, "PTA1/LLWU_P0/RF_ANT_B", "LPUART0_CTS", "LPI2C0_SDAS", "LPUART1_CTS", x, x, "JTAG_TCLK/SWD_CLK"),
    PTA2: (pta2, 2, PCR2, ALT7), ("PTA2/LLWU_P1", "F11"), (x, "PTA2/LLWU_P1/RF_ANT_A", "LPUART0_RX", "LPI2C0_SDA", "LPUART1_RX", x, x, "JTAG_TDI"),
    PTA3: (pta3, 3, PCR3, ALT7), ("PTA3", "D11"), (x, "PTA3/RF0_TX_SWITCH", "LPUART0_TX", "LPI2C0_SCL", "LPUART1_TX", x, "TPM0_CLKIN", "JTAG_TDO/SWD_SWO"),
    PTA4: (pta4, 4, PCR4, ALT7), ("PTA4", "B9"), (x, "PTA4/RF0_RX_SWITCH", "LPUART0_RTS", "LPI2C0_SCLS", "LPUART1_RTS", x, "LPCMP0_OUT", "JTAG_TMS/SWD_DIO"),
    PTA9: (pta9, 9, PCR9, ALT7), ("PTA9", "E10"), (x, "PTA9", "LPI2C2_SDAS", "LPSPI3_SCK", x, "FB_A23", x, "RV_JTAG_TACK"),
    PTA10: (pta10, 10, PCR10, ALT7), ("PTA10", "A9"), (x, "PTA10", "LPI2C2_SCLS", "LPSPI3_SOUT", x, "FB_A22", x, "RV_JTAG_TDI"),
    PTA14: (pta14, 14, PCR14, ALT7), ("PTA14", "E8"), (x, "PTA14", "LPi2C2_SDA", x, x, "FB_AD23", "LPCMP0_OUT", "RV_JTAG_TDO"),
    PTA15: (pta15, 15, PCR15, ALT7), ("PTA15", "A7"), (x, "PTA15", "LPI2C2_SCL", x, x, "FB_AD22", x, "RV_JTAG_TMS"),
    PTA17: (pta17, 17, PCR17, ALT0), ("PTA17", "F7"), (x, "PTA17", "LPI2C2_HREQ", "LPSPI3_PCS1", "EMVSIM0_CLK", "FB_AD21", x, x),
    PTA18: (pta18, 18, PCR18, ALT0), ("PTA18", "D8"), (x, "PTA18", "LPSPI2_PCS1", "LPSPI3_PCS3", "EMVSIM0_RST", "FB_AD20", x, x),
    PTA19: (pta19, 19, PCR19, ALT0), ("PTA19", "D7"), (x, "PTA19", "LPSPI2_PCS3", "LPSPI3_SCK", "EMVSIM0_VCCEN", "FB_AD19", "TPM2_CH5", x),
    PTA20: (pta20, 20, PCR20, ALT0), ("PTA20", "C7"), (x, "PTA20", "LPSPI2_SCK", "LPSPI1_PCS1", "EMVSIM0_IO", "FB_AD18", "TPM2_CH4", x),
    PTA21: (pta21, 21, PCR21, ALT0), ("PTA21", "B7"), (x, "PTA21", "LPSPI2_SOUT", x, "EMVSIM0_PD", "FB_AD17", "TPM2_CH3", x),
    PTA22: (pta22, 22, PCR22, ALT0), ("PTA22/LLWU_P2", "B6"), (x, "PTA22/LLWU_P2", "LPSPI2_PCS2", x, "LPI2C2_HREQ", "FB_AD16", "TPM2_CH2", x),
    PTA23: (pta23, 23, PCR23, ALT0), ("PTA23", "E6"), (x, "PTA23", "LPSPI2_SIN", "LPSPI1_PCS3", "LPI2C2_SDA", "FB_AD15", "TPM2_CH1", x),
    PTA24: (pta24, 24, PCR24, ALT0), ("PTA24", "D6"), (x, "PTA24", "LPSPI2_PCS0", "LPSPI1_SCK", "LPI2C2_SCL", "FB_OE_b", "TPM2_CH0", x),
    PTA25: (pta25, 25, PCR25, ALT0), ("PTA25", "B5"), (x, "PTA25", "LPUART1_RX", "LPSPI3_SOUT", "LPI2C2_SDAS", "FB_AD31", x, x),
    PTA26: (pta26, 26, PCR26, ALT0), ("PTA26", "A5"), (x, "PTA26", "LPUART1_TX", "LPSPI3_PCS2", "LPI2C2_SCLS", "FB_AD30", x, x),
    PTA27: (pta27, 27, PCR27, ALT0), ("PTA27", "A3"), (x, "PTA27", "LPUART1_CTS", "LPSPI3_SIN", x, "FB_AD29", x, x),
    PTA28: (pta28, 28, PCR28, ALT0), ("PTA28", "A2"), (x, "PTA28", "LPUART1_RTS", "LPSPI3_PCS0", x, "FB_AD28", x, x),
    PTA30: (pta30, 30, PCR30, ALT0), ("PTA30/LLWU_P3", "A1/B2"), (x, "PTA30/LLWU_P3", "LPUART2_CTS", "LPSPI1_SOUT", x, "FB_AD14", "TPM1_CH0", "LPTMR2_ALT2"),
    PTA31: (pta31, 31, PCR31, ALT0), ("PTA31", "C4"), (x, "PTA31", "LPUART2_RTS", "LPSPI1_PCS2", x, "FB_AD13", "TPM1_CH1", x),
] }

port_impl! { PORTB, portb, PTBx, [
    PTB0: (ptb0, 0, PCR0, ALT0), ("PTB0", "B3"), (x, "PTB0", "LPUART2_TX", "LPSPI1_SIN", "USB0_SOF_OUT", "CLKOUT", "TPM1_CLKIN", x),
    PTB1: (ptb1, 1, PCR1, ALT0), ("PTB1/LLWU_P4", "C3"), (x, "PTB1/LLWU_P4", "LPUART2_RX", "LPSPI1_PCS0", "I2S0_TX_D1", "FB_AD12", x, "LPTMR2_ALT3"),
    PTB2: (ptb2, 2, PCR2, ALT0), ("PTB2/LLWU_P5", "B10"), (x, "PTB2/LLWU_P5/RF0_RF_OSC_EN", "LPSPI0_PCS1", "LPUART1_RX", "I2S0_TX_D0", "FB_AD11", "TPM0_CH0", x),
    PTB3: (ptb3, 3, PCR3, ALT0), ("PTB3", "C1"), ("LPADC0_SE0", "PTB3/RF0_EXT_OSC_EN", "LPSPI0_PCS3", "LPUART1_TX", "I2S0_TX_FS", "FB_AD10", "TPM0_CH1", x),
    PTB4: (ptb4, 4, PCR4, ALT0), ("PTB4/LLWU_P6", "C2"), ("LPADC0_SE1", "PTB4/LLWU_P6/RF0_RF_OFF/RF0_DFT_RESET", "LPSPI0_SCK", "LPUART1_CTS", "I2S0_TX_BCLK", "FB_AD9", "TPM0_CH2", x),
    PTB5: (ptb5, 5, PCR5, ALT0), ("PTB5", "D2"), (x, "PTB5/RF0_ACTIVE", "LPSPI0_SOUT", "LPUART1_RTS", "I2S0_MCLK", "FB_AD8", "TPM0_CH3", x),
    PTB6: (ptb6, 6, PCR6, ALT0), ("PTB6/LLWU_P7", "E1"), (x, "PTB6/LLWU_P7", "LPSPI0_PCS2", "LPI2C1_SDA", "I2S0_RX_BCLK", "FB_AD7", "TPM0_CH4", "RF0_BSM_FRAME"),
    PTB7: (ptb7, 7, PCR7, ALT0), ("PTB7/LLWU_P7", "E2"), ("LPADC0_SE2", "PTB7/LLWU_P8", "LPSPI0_SIN", "LPI2C1_SDAS", "I2S0_RX_FS", "FB_AD6", "TPM0_CH5", "RF0_BSM_DATA"),
    PTB8: (ptb8, 8, PCR8, ALT0), ("PTB8/LLWU_P9", "F5"), (x, "PTB8/LLWU_P9/RF0_EARLY_WARNING", "LPSPI0_PCS0", "LPI2C1_SCLS", "I2S0_RX_D0", "DB_AD5", x, "LPTMR0_ALT1"),
    PTB9: (ptb9, 9, PCR9, ALT0), ("PTB9", "F4"), ("LPADC0_SE3", "PTB9/SPM_LPREQ", "LPSPI0_PCS1", "LPI2C1_SCL", "I2S0_RX_D1", "FB_RW_b", x, "FXIO0_D0"),
    PTB11: (ptb11, 11, PCR11, ALT0), ("PTB11", "G6"), (x, "PTB11", "LPUART2_RX", "LPI2C1_SDAS", "LPI2C0_SDA", "FB_AD27", x, "FXIO0_D1"),
    PTB12: (ptb12, 12, PCR12, ALT0), ("PTB12", "G4"), (x, "PTB12", "LPUART2_TX", "LPI2C1_SCLS", "LPI2C0_SCL", "FB_AD26", "TPM3_CLKIN", "FXIO0_D2"),
    PTB13: (ptb13, 13, PCR13, ALT0), ("PTB13", "G3"), (x, "PTB13", "LPUART2_CTS", "LPI2C1_SDA", "LPI2C0_SDAS", "FB_AD25", "TPM3_CH0", "FXIO0_D3"),
    PTB14: (ptb14, 14, PCR14, ALT0), ("PTB14", "G2"), (x, "PTB14", "LPUART2_RTS", "LPI2C1_SCL", "LPI2C0_SCLS", "FB_AD24", "TPM3_CH1", "FXIO0_D4"),
    PTB15: (ptb15, 15, PCR15, ALT0), ("PTB15", "G1"), (x, "PTB15", x, "LPI2C1_HREQ", "LPI2C3_SCL", "FB_CS5_b/FB_TSIZ1/FB_BE23_16_b", "TPM0_CLKIN", "FXIO0_D5"),
    PTB16: (ptb16, 16, PCR16, ALT0), ("PTB16/LLWU_P10", "H5"), (x, "PTB16/LLWU_P10", x, "LPUART3_CTS", "LPI2C3_SDA", "FB_CS4_b/FB_TSIZ0/FB_BE31_24_b", x, "FXIO0_D6"),
    PTB17: (ptb17, 17, PCR17, ALT0), ("PTB17", "K5"), (x, "PTB17", x, "LPUART3_RTS", "LPI2C3_SCLS", "FB_TBST_b/FB_CS2_b/FB_BE15_8_b", x, "FXIO0_D7"),
    PTB18: (ptb18, 18, PCR18, ALT0), ("PTB18", "H2"), (x, "PTB18", "LPSPI1_PCS1", "LPUART2_RX", "LPI2C3_SDAS", "FB_CS3_b/FB_BE7_0_b", "FB_TA_b", "FXIO0_D8"),
    PTB19: (ptb19, 19, PCR19, ALT0), ("PTB19", "K4"), (x, "PTB19", "LPSPI1_PCS3", "LPUART2_TX", x, "FB_ALE/FB_CS1_b/FB_TS_b", "TPM1_CLKIN", "FXIO0_D9"),
    PTB20: (ptb20, 20, PCR20, ALT0), ("PTB20/LLWU_P11", "J1"), (x, "PTB20/LLWU_P11", "LPSPI1_SCK", "LPUART2_CTS", x, "FB_CS0_b", "TPM1_CH0", "FXIO0_D10"),
    PTB21: (ptb21, 21, PCR21, ALT0), ("PTB21", "J2"), (x, "PTB21", "LPSPI1_SOUT", "LPUART2_RTS", "LPI2C2_HREQ", "FB_AD4", "TPM1_CH1", "FXIO0_D11"),
    PTB22: (ptb22, 22, PCR22, ALT0), ("PTB22/LLWU_P12", "L1"), (x, "PTB22/LLWU_P12", "LPSPI1_PCS2", "LPUART0_CTS", "LPI2C2_SDA", "FB_AD3", "TPM2_CLKIN", "FXIO0_D12"),
    PTB24: (ptb24, 24, PCR24, ALT0), ("PTB24", "L2"), (x, "PTB24", "LPSPI1_SIN", "LPUART0_RTS", "LPI2C2_SCL", "FB_AD2", "EWM_IN", "FXIO0_D13"),
    PTB25: (ptb25, 25, PCR25, ALT0), ("PTB25/LLWU_P13", "L6"), (x, "PTB25/LLWU_P13", "LPSPI1_PCS0", "LPUART0_RX", "LPI2C2_SDAS", "FB_AD1", "EWM_OUT_b", "FXIO0_D14"),
    PTB26: (ptb26, 26, PCR26, ALT0), ("PTB26", "L4"), (x, "PTB26", "USB0_SOF_OUT", "LPUART0_TX", "LPI2C2_SCLS", "FB_AD0", "LPCMP0_OUT", "RF0_BSM_CLK"),
    PTB28: (ptb28, 28, PCR28, ALT0), ("PTB28/LLWU_P14", "M4"), (x, "PTB28/LLWU_P14", x, "LPUART3_RX", "I2S0_TX_D0", "FB_A16", x, "FXIO0_D15"),
    PTB29: (ptb29, 29, PCR29, ALT0), ("PTB29", "L3"), (x, "PTB29", x, "LPUART3_TX", "I2S0_TX_FS", "FB_A17", x, "FXIO0_D16"),
    PTB30: (ptb30, 30, PCR30, ALT0), ("PTB30", "M5"), (x, "PTB30", x, x, "I2S0_TX_BCLK", "FB_A18", x, x),
    PTB31: (ptb31, 31, PCR31, ALT0), ("PTB31", "M7"), (x, "PTB31", x, x, "I2S0_RX_D0", "FB_A19", x, x),
] }

// PTC26 is internally connected but bound to no output pins
// P60, RV32M1DS
// PTC26: (ptc26, 26, PCR26, ALT0),
port_impl! { PORTC, portc, PTCx, [
    PTC0: (ptc0, 0, PCR0, ALT0), ("PTC0", "N1"), (x, "PTC0", x, x, "I2S0_RX_FS", "FB_A20", x, x),
    PTC1: (ptc1, 1, PCR1, ALT0), ("PTC1", "M2"), (x, "PTC1", x, x, "I2S0_RX_BCLK", "FB_A21", x, x),
    PTC7: (ptc7, 7, PCR7, ALT0), ("PTC7/LLWU_P15", "N2"), ("LPCMP0_IN0", "PTC7/LLWU_P15", "LPSPI0_PCS3", "LPUART0_RX", "LPI2C1_HREQ", x, "TPM0_CH0", "LPTMR0_ALT1"),
    PTC8: (ptc8, 8, PCR8, ALT0), ("PTC8", "P3"), ("LPCMP0_IN1", "PTC8", "LPSPI0_SCK", "LPUART0_TX", "LPI2C0_HREQ", x, "TPM0_CH1", x),
    PTC9: (ptc9, 9, PCR9, ALT0), ("PTC9/LLWU_P16", "R1"), ("LPADC0_SE4/LPCMP0_IN2", "PTC9/LLWU_P16", "LPSPI0_SOUT", "LPUART0_CTS", "LPI2C0_SDA", x, "TPM0_CH2", "LPTMR0_ALT2"),
    PTC10: (ptc10, 10, PCR10, ALT0), ("PTC10", "R2"), ("LPADC0_SE5", "PTC10", "LPSPI0_PCS2", "LPUART0_RTS", "LPI2C0_SCL", x, "TPM0_CH3", x),
    PTC11: (ptc11, 11, PCR11, ALT0), ("PTC11/LLWU_P17", "T1"), ("LPADC0_SE6", "PTC11/LLWU_P17", "LPSPI0_SIN", "LPI2C1_SDA", "LPI2C0_SDAS", x, "TPM0_CH4", "EWM_IN"),
    PTC12: (ptc12, 12, PCR12, ALT0), ("PTC12/LLWU_P18", "R3"), ("LPADC0_SE7", "PTC12/LLWU_P18", "LPSPI0_PCS0", "LPI2C1_SCL", "LPI2C0_SCLS", x, "TPM0_CH5", "EWM_OUT_b"),
    PTC27: (ptc27, 27, PCR27, ALT0), ("PTC27", "P6"), (x, "PTC27", x, x, x, x, "TPM0_CH4", x),
    PTC28: (ptc28, 28, PCR28, ALT0), ("PTC28", "U5"), (x, "PTC28", x, "LPSPI0_PCS1", x, x, "TPM0_CH3", "FXIO0_D17"),
    PTC29: (ptc29, 29, PCR29, ALT0), ("PTC29", "N6"), (x, "PTC29", "LPUART1_RX", "LPSPI0_PCS3", x, x, "TPM0_CH2", "FXIO0_D18"),
    PTC30: (ptc30, 30, PCR30, ALT0), ("PTC30", "R7"), (x, "PTC30", "LPUART1_TX", "LPSPI0_SCK", x, x, "TPM0_CH1", "FXIO0_D19"),
] }

port_impl! { PORTD, portd, PTDx, [
    PTD0: (ptd0, 0, PCR0, ALT0), ("PTD0", "T7"), (x, "PTD0", "LPUART1_CTS", "LPSPI0_SOUT", x, x, "TPM0_CH0", "FXIO0_D20"),
    PTD1: (ptd1, 1, PCR1, ALT0), ("PTD1", "P7"), (x, "PTD1", "LPUART1_RTS", "LPSPI0_PCS2", x, x, "EWM_IN", "FXIO0_D21"),
    PTD2: (ptd2, 2, PCR2, ALT0), ("PTD2", "U7"), (x, "PTD2", "SDHC0_D7", "LPSPI0_SIN", x, x, "EWM_OUT_b", "FXIO0_D22"),
    PTD3: (ptd3, 3, PCR3, ALT0), ("PTD3", "T8"), (x, "PTD3", "SDHC0_D6", "LPSPI0_PCS0", "EMVSIM0_CLK", x, "TPM2_CLKIN", "FXIO0_D23"),
    PTD4: (ptd4, 4, PCR4, ALT0), ("PTD4", "N8"), (x, "PTD4", "SDHC0_D5", "LPSPI2_PCS1", "EMVSIM0_RST", x, x, "FXIO0_D24"),
    PTD5: (ptd5, 5, PCR5, ALT0), ("PTD5", "N10"), ("LPADC0_SE8", "PTD5", "SDHC0_D4", "LPSPI2_PCS3", "EMVSIM0_VCCEN", x, x, "FXIO0_D25"),
    PTD6: (ptd6, 6, PCR6, ALT0), ("PTD6", "U9"), ("LPADC0_SE9", "PTD6", "SDHC0_D1", "LPSPI2_SCK", "EMVSIM0_IO", "TRACE_D3", "TPM2_CH5", "FXIO0_D26"),
    PTD7: (ptd7, 7, PCR7, ALT0), ("PTD7", "P10"), ("LPADC0_SE10", "PTD7", "SDHC0_D0", "LPSPI2_SOUT", "EMVSIM0_PD", "TRACE_D2", "TPM2_CH4", "FXIO0_D27"),
    PTD8: (ptd8, 8, PCR8, ALT0), ("PTD8/LLWU_P19", "T9"), ("LPADC0_SE11", "PTD8/LLWU_P19", "SDHC0_DCLK", "LPSPI2_PCS2", "LPI2C1_SDAS", "TRACE_D1", "TPM2_CH3", "FXIO0_D28"),
    PTD9: (ptd9, 9, PCR9, ALT0), ("PTD9", "U11"), ("LPADC0_SE12", "PTD9", "SDHC0_CMD", "LPSPI2_SIN", "LPI2C1_SCLS", "TRACE_D0", "TPM2_CH2", "FXIO0_D29"),
    PTD10: (ptd10, 10, PCR10, ALT0), ("PTD10/LLWU_P20", "P11"), ("LPADC0_SE13", "PTD10/LLWU_P20", "SDHC0_D3", "LPSPI2_PCS0", "LPI2C1_SDA", "TRACE_CLKOUT", "TPM2_CH1", "FXIO0_D30"),
    PTD11: (ptd11, 11, PCR11, ALT0), ("PTD11", "R11"), ("LPADC0_SE14", "PTD11", "SDHC0_D2", "USB_SOF_OUT", "LPI2C1_SCL", "CLKOUT", "TPM2_CH0", "FXIO0_D31"),
] }
