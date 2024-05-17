
//! This is an emulator for the CHIP 8, written in Rust.

use std::{fs::File, io::{self, Read}};
use chip_8_emulator::chip_8::{self, drivers, TimedRunner, KeyboardDriver};

/// This is the main function for the emulatort
fn main() {
    println!("Starting emulator");

    let program_file = File::open("programs/IBM Logo.ch8").expect("unable to find the program file specified");

    let mut chip_8_system = TimedRunner::new(drivers::TerminalNumbers, drivers::TerminalBeep, drivers::KeySender::new());
    chip_8_system.init(program_file);

    let driver = drivers::KeySender::new();
    loop {
	chip_8_system.decode_next_timed(100f64);
    }
}
