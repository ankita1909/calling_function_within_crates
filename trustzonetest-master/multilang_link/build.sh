#!/bin/sh

(cd hello && cargo build --release)
arm-none-eabi-as -c -mthumb -mlittle-endian -march=armv7-m -mcpu=cortex-m3 startup.S -o startup.o
arm-none-eabi-gcc -c -mthumb -ffreestanding -mlittle-endian -march=armv7-m -mcpu=cortex-m3 test.c -o test.o
arm-none-eabi-ld -T link.x test.o startup.o hello/target/thumbv8m.main-none-eabihf/release/libhello.a -o test.elf

#clang -v --target=arm-none-eabi -march=armv7-m -mthumb -mfloat-abi=soft -mcpu=cortex-m3 startup.S test.c
#lang -v --target=arm-none-eabi -march=armv7-m -mthumb -mfloat-abi=soft -mcpu=cortex-m3 test.o startup.o -o test.elf
