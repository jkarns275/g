use std::process::Command;

fn main() {
    // Command::new("xargo")
        // .args(&["build", "--target", "gba", "--release"])
        // .status().unwrap();
    Command::new("arm-none-eabi-ld")
        .args(&["-T", "linker.ld", "-o", "out/g.elf", "target/gba/release/g.a"])
        .status().unwrap();
    Command::new("arm-none-eabi-objcopy")
        .args(&["-O", "binary", "out/g.elf", "out/g.gba"])
        .status().unwrap();
    //Command::new("gbafix")
}