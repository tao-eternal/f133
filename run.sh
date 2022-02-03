#! /usr/bin/bash

# The compilation process may get stuck, please run it separately
# cargo objcopy -- -O binary --strip-all f133.bin

xfel ddr ddr2
xfel write 0x40000000 f133.bin
xfel exec 0x40000000