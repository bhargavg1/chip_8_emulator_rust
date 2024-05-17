
//! This is an emulator for the CHIP 8, written in Rust.

use std::fs::File;
use chip_8_emulator::chip_8::{TimedRunner, drivers};

/// This is the main function for the emulatort
fn main() {
    println!("Starting emulator");

    let program_file = File::open("programs/IBM Logo.ch8").expect("unable to find the program file specified");

    let mut chip_8_system = TimedRunner::new(drivers::TerminalNumbers::new(), drivers::TerminalBeep::new());
    chip_8_system.init(program_file);

    loop {
	chip_8_system.loop_through_all_instructions(100f64);
    }
}
