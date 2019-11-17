# author:  Luo Jia <luojia65@hust.edu.cn> 2019-11-17
.text
.section .vectors, "ax"
.option norvc
.globl _rv32m1_vectors
_rv32m1_vectors:
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
    jal x0, reset_handler
    # Illegal instrution exception
    jal x0, illegal_instruction_handler
    # ecall handler
    jal x0, ecall_handler
    # LSU error
    jal x0, lsu_handler

.text
.section .text, "ax"
reset_handler:
    j _start
illegal_instruction_handler:
    j _start
ecall_handler:
    j _start
lsu_handler:
    j _start
