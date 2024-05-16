
//! This is an emulator for the CHIP 8, written in Rust.

use std::fs::File;

use std::time::Duration;
use std::thread;
use chip_8_emulator::chip_8::{Chip8, Timable};

/// This is the main function for the emulatort
fn main() {
    println!("Starting emulator");

    let program = File::open("programs/IBM Logo.ch8").expect("Error opening file");
    let mut chip8_system = Chip8::new();
    chip8_system.init(program);

    let counter = 0;
    loop {
	chip8_system.do_act(counter, 2000);

	thread::sleep(Duration::from_micros(500));
    }
}
