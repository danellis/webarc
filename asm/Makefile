all: test.bin

test.elf: test.asm
	arm-none-eabi-as -o test.elf test.asm

test.bin: test.elf
	arm-none-eabi-objcopy -O binary --only-section .text test.elf test.bin
