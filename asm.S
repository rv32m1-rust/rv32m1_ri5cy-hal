# author:  Luo Jia <luojia65@hust.edu.cn> 2019-11-17
.text
.section .vectors, "ax"
.option norvc
    # todo!
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    jal x0, abort
    # reset vector
    jal x0, Reset_Handler
    # Illegal instrution exception
    jal x0, IllegalInstruction_Handler
    # ecall handler
    jal x0, Ecall_Handler
    # LSU error
    jal x0, LSU_Handler

.text
.section .text, "ax"
Reset_Handler:
    j _start
