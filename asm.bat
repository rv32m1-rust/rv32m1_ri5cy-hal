riscv64-unknown-elf-gcc asm.S -o bin/riscv32imc-unknown-none-elf.o -march=rv32imc -mabi=ilp32
wsl ar rcs bin/riscv32imc-unknown-none-elf.a bin/riscv32imc-unknown-none-elf.o
wsl rm bin/*.o
