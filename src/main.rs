
//! This is an emulator for the CHIP 8, written in Rust.

use std::env;
use std::fs::File;
use chip_8_emulator::chip_8::{drivers, TimedRunner};

/// This is the main function for the emulatort
fn main() {
    println!("Starting emulator");
    let args = env::args().collect::<Vec<String>>();
    let input_file_name = {
	if args.len() < 2 {
	    panic!("program file location not provided");
	} else {
	    &args[1]
	}
    };
    
    println!("taking input program location from provided 1st argument");
    let program_file = File::open(input_file_name).expect("unable to find the program file specified");

    println!("initializing chip8 decoder");
    let mut chip_8_system = TimedRunner::new(drivers::TerminalNumbers, drivers::TerminalBeep, drivers::KeySender::new());
    
    println!("loading program...");
    chip_8_system.init(program_file);

    println!("starting decode loop");
    loop {
	chip_8_system.decode_next_timed(1f64);
    }
}
