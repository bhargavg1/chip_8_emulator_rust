
//! This is an emulator for the CHIP 8, written in Rust.
//! I made this project mainly as a way to learn about Rust and it's features.
//! Along the way, I definetly feel like I have gotten a hang of the basics of how rust works, suct as match statements,
//!  ownership, unsafe blocks, etc. I didnt delve much into threads or macros though, maybe thats for another project.
//!
//! To start this program, you can just run "cargo run -- /path/to/.ch8/program" in order to run a specific program.

use std::env;
use std::fs::File;
use chip_8_emulator::chip_8::{drivers, TimedRunner};

///This is the main function for the emulator
///the chip8's internal workings are all abstracted away into other modules, so this main function can be self expalnatory.
fn main() {
    println!("Starting emulator");
    let args = env::args().collect::<Vec<String>>();
    let input_file_name = {
	if args.len() < 2 {
	    panic!("program file location not provided, provide path to .ch8 program as first argument to this program to run it");
	} else {
	    &args[1]
	}
    };
    
    println!("taking input program location from provided 1st argument");
    let program_file = File::open(input_file_name).expect("unable to find the program file specified");

    println!("initializing chip8 decoder");
    let mut chip_8_system = TimedRunner::new(drivers::StdoutDisplay::new(), drivers::TerminalBeep::new(), drivers::StdinKeysender::new()); 
    
    println!("loading program...");
    chip_8_system.init(program_file);

    println!("starting decode loop");
    loop {
	chip_8_system.decode_next_timed(1f64);
    }
}
