/* 
    RV32M1_RI5CY-HAL linker script 
    Author: Luo Jia <luojia65@hust.edu.cn> 2019-11-18
*/

/* Define memory regions, used in region alias part and sections. */
MEMORY
{
    /* 0x0000_0000 - 0x000F_FFFF: M4 - 1 MB flash 
        (including exception vectors in first 1024 bytes) */
    FLASH           (RX) : ORIGIN = 0x00000000, LENGTH = 1M - 256
    /* Interrupt assignment to RI5CY begins at 0x000F_FF00 */
    RI5CY_VECTORS   (RX) : ORIGIN = 0x000FFF00, LENGTH = 256
    /* 0x0800_0000 - 0x0800_FFFF: Cortex M4 ITCM SRAM - 64 KB*/
    RAM_LOW         (RW) : ORIGIN = 0x08000000, LENGTH = 64K
    /* 0x2000_0000 - 0x2002_FFFF: Cortex M4 DTCM SRAM - 192 KB */
    RAM_HIGH        (RW) : ORIGIN = 0x20000000, LENGTH = 192K
    /* 0x4801_0000 = 0x4801_07FF: USB SRAM - 2 KB */
    USB_SRAM        (RW) : ORIGIN = 0x48010000, LENGTH = 2K
}

/* Alias region for underlying `riscv-rt` crate. */
REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM_LOW);
REGION_ALIAS("REGION_BSS", RAM_LOW);
REGION_ALIAS("REGION_STACK", RAM_LOW);
REGION_ALIAS("REGION_HEAP", RAM_HIGH);

/* Import vectors and handlers, assuming we need them in the output. */
/* Do not delete - or the linker won't know we need to link the vectors */
/* This symbol is defined in `asm.S` file */
EXTERN(_rv32m1_ri5cy_vectors);
/* Use default handler (defined in `asm.S`) for default interrupt handlers */
EXTERN(DefaultHandler);

/* Provide interrupt handlers for `_rv32m1_ri5cy_vectors` part in `asm.S` */
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
/* Provide finxed-priority internal exception and reset handlers */ 
/* The `Reset_Handler` is the entry point of RI5CY code or where initial PC points */
/* RI5CY Reset, Initial PC */
PROVIDE(Reset_Handler = DefaultHandler);
/* RI5CY Illegal Instruction */
PROVIDE(IllegalInstruction_Handler = DefaultHandler);
/* RI5CY ECALL Instruction Executed */
PROVIDE(Ecall_Handler = DefaultHandler);
/* RI5CY Load Store Unit Error */
PROVIDE(LSU_Handler = DefaultHandler);

/* Extra sections we need to link in addition to what `riscv-rt` provided */
SECTIONS {
    /* Interrupt vector section */
    /* The section `.vectors` is used in `asm.S` file */
    /* Symbol `RI5CY_VECTORS` is defined in `MEMORY` part of this linker script file */
    .vectors : 
    {
        /* `KEEP` means the vectors is somehow used and shouldn't be optimized out */
        KEEP (*(.vectors))
    } > RI5CY_VECTORS
}
