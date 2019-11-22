; author:  Luo Jia <luojia65@hust.edu.cn> 2019-11-17
.text
.section .vectors, "ax"
.option norvc ; Do not compile as 16-bit wide instructions 
.globl _rv32m1_ri5cy_vectors
_rv32m1_ri5cy_vectors:
    jal x0, DMA0_0_4_8_12_IRQHandler
    jal x0, DMA0_1_5_9_13_IRQHandler
    jal x0, DMA0_2_6_10_14_IRQHandler
    jal x0, DMA0_3_7_11_15_IRQHandler
    jal x0, DMA0_Error_IRQHandler
    jal x0, CMC0_IRQHandler
    jal x0, MUA_IRQHandler
    jal x0, USB0_IRQHandler
    jal x0, USDHC0_IRQHandler
    jal x0, I2S0_IRQHandler
    jal x0, FLEXIO0_IRQHandler
    jal x0, EMVSIM0_IRQHandler
    jal x0, LPIT0_IRQHandler
    jal x0, LPSPI0_IRQHandler
    jal x0, LPSPI1_IRQHandler
    jal x0, LPI2C0_IRQHandler
    jal x0, LPI2C1_IRQHandler
    jal x0, LPUART0_IRQHandler
    jal x0, PORTA_IRQHandler
    jal x0, TPM0_IRQHandler
    jal x0, LPDAC0_IRQHandler
    jal x0, ADC0_IRQHandler
    jal x0, LPCMP0_IRQHandler
    jal x0, RTC_IRQHandler
    jal x0, INTMUX0_0_IRQHandler
    jal x0, INTMUX0_1_IRQHandler
    jal x0, INTMUX0_2_IRQHandler
    jal x0, INTMUX0_3_IRQHandler
    jal x0, INTMUX0_4_IRQHandler
    jal x0, INTMUX0_5_IRQHandler
    jal x0, INTMUX0_6_IRQHandler
    jal x0, INTMUX0_7_IRQHandler
    ; reset vector
    jal x0, Reset_Handler
    ; Illegal instrution exception
    jal x0, IllegalInstruction_Handler
    ; ecall handler
    jal x0, Ecall_Handler
    ; LSU error
    jal x0, LSU_Handler

.text
.section .text, "ax"
.globl DefaultHandler
DefaultHandler:
    j _start

.section .trap, "ax"

.align 2
.global irq_entry
.type irq_entry,@function
irq_entry:
    addi sp, sp, -16*REGBYTES

    STORE ra, 0*REGBYTES(sp)
    STORE t0, 1*REGBYTES(sp)
    STORE t1, 2*REGBYTES(sp)
    STORE t2, 3*REGBYTES(sp)
    STORE t3, 4*REGBYTES(sp)
    STORE t4, 5*REGBYTES(sp)
    STORE t5, 6*REGBYTES(sp)
    STORE t6, 7*REGBYTES(sp)
    STORE a0, 8*REGBYTES(sp)
    STORE a1, 9*REGBYTES(sp)
    STORE a2, 10*REGBYTES(sp)
    STORE a3, 11*REGBYTES(sp)
    STORE a4, 12*REGBYTES(sp)
    STORE a5, 13*REGBYTES(sp)
    STORE a6, 14*REGBYTES(sp)
    STORE a7, 15*REGBYTES(sp)

    ; todo!

    LOAD ra, 0*REGBYTES(sp)
    LOAD t0, 1*REGBYTES(sp)
    LOAD t1, 2*REGBYTES(sp)
    LOAD t2, 3*REGBYTES(sp)
    LOAD t3, 4*REGBYTES(sp)
    LOAD t4, 5*REGBYTES(sp)
    LOAD t5, 6*REGBYTES(sp)
    LOAD t6, 7*REGBYTES(sp)
    LOAD a0, 8*REGBYTES(sp)
    LOAD a1, 9*REGBYTES(sp)
    LOAD a2, 10*REGBYTES(sp)
    LOAD a3, 11*REGBYTES(sp)
    LOAD a4, 12*REGBYTES(sp)
    LOAD a5, 13*REGBYTES(sp)
    LOAD a6, 14*REGBYTES(sp)
    LOAD a7, 15*REGBYTES(sp)

    addi sp, sp, 16*REGBYTES
    mret
