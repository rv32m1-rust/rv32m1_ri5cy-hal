target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# flash the program into the chip
load

# skip reset handler function
stepi
