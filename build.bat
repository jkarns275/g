xargo build --target gba --release
arm-none-eabi-as -o out/begin.o src/begin.s
arm-none-eabi-ld -T linker.ld -o out/g.elf out/begin.o target/gba/release/libg.a
arm-none-eabi-objcopy -O binary out/g.elf out/g.gba
gbafix out/g.gba