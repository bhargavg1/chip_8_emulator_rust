
//! This is an emulator for the CHIP 8, written in Rust.

use std::fs::File;
use std::io;
use chip_8_emulator::chip_8::{drivers, TimedRunner};

/// This is the main function for the emulatort
fn main() {
    println!("Starting emulator");

    let mut input_file_name = String::new();
    println!("input the program location...\t");
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input_file_name);
    let _ = input_file_name.remove(input_file_name.len() - 1);
    
    let program_file = File::open(input_file_name).expect("unable to find the program file specified");

    let mut chip_8_system = TimedRunner::new(drivers::TerminalNumbers, drivers::TerminalBeep, drivers::KeySender::new());
    chip_8_system.init(program_file);

    //let mut throwaway = String::new();
    loop {
	//let _ = stdin.read_line(&mut throwaway);
	chip_8_system.decode_next_timed(1f64);
    }
}
