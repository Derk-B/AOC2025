# !/bin/bash
nasm -f elf64 main.asm -o build/main.o;
nasm -f elf64 debug.asm -o build/debug.o;
ld build/main.o build/debug.o -o build/main; ./main