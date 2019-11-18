MEMORY
{
    RI5CY_VECTORS   (RX) : ORIGIN = 0x000FFF00, LENGTH = 256
    FLASH           (RX) : ORIGIN = 0x00000000, LENGTH = 1M - 256
    RAM_LOW         (RW) : ORIGIN = 0x08000000, LENGTH = 64K
    RAM_HIGH        (RW) : ORIGIN = 0x20000000, LENGTH = 192K
    USB_SRAM        (RW) : ORIGIN = 0x48010000, LENGTH = 0x00000800
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM_LOW);
REGION_ALIAS("REGION_BSS", RAM_LOW);
REGION_ALIAS("REGION_STACK", RAM_LOW);
REGION_ALIAS("REGION_HEAP", RAM_HIGH);

EXTERN(_rv32m1_ri5cy_vectors);
EXTERN(DefaultHandler);

PROVIDE(DMA0_0_4_8_12_IRQHandler = DefaultHandler);
PROVIDE(DMA0_1_5_9_13_IRQHandler = DefaultHandler);
PROVIDE(DMA0_2_6_10_14_IRQHandler = DefaultHandler);
PROVIDE(DMA0_3_7_11_15_IRQHandler = DefaultHandler);
PROVIDE(DMA0_Error_IRQHandler = DefaultHandler);
PROVIDE(CMC0_IRQHandler = DefaultHandler);
PROVIDE(MUA_IRQHandler = DefaultHandler);
PROVIDE(USB0_IRQHandler = DefaultHandler);
PROVIDE(USDHC0_IRQHandler = DefaultHandler);
PROVIDE(I2S0_IRQHandler = DefaultHandler);
PROVIDE(FLEXIO0_IRQHandler = DefaultHandler);
PROVIDE(EMVSIM0_IRQHandler = DefaultHandler);
PROVIDE(LPIT0_IRQHandler = DefaultHandler);
PROVIDE(LPSPI0_IRQHandler = DefaultHandler);
PROVIDE(LPSPI1_IRQHandler = DefaultHandler);
PROVIDE(LPI2C0_IRQHandler = DefaultHandler);
PROVIDE(LPI2C1_IRQHandler = DefaultHandler);
PROVIDE(LPUART0_IRQHandler = DefaultHandler);
PROVIDE(PORTA_IRQHandler = DefaultHandler);
PROVIDE(TPM0_IRQHandler = DefaultHandler);
PROVIDE(LPDAC0_IRQHandler = DefaultHandler);
PROVIDE(ADC0_IRQHandler = DefaultHandler);
PROVIDE(LPCMP0_IRQHandler = DefaultHandler);
PROVIDE(RTC_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_0_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_1_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_2_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_3_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_4_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_5_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_6_IRQHandler = DefaultHandler);
PROVIDE(INTMUX0_7_IRQHandler = DefaultHandler);

PROVIDE(Reset_Handler = DefaultHandler);
PROVIDE(IllegalInstruction_Handler = DefaultHandler);
PROVIDE(Ecall_Handler = DefaultHandler);
PROVIDE(LSU_Handler = DefaultHandler);

SECTIONS {
    .vectors : 
    {
        KEEP (*(.vectors))
    } > RI5CY_VECTORS
}
