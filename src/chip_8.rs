
//! contains the various modules which implement the features of the chip 8.

mod memory;
mod video;
mod timers;
mod instruction_decoder;

pub use timers::Timable;

use std::fs::File;
use std::io::{BufReader, Read};


///The full system which implements all the features of the Chip8.
///It takes in a program and loads it into memory, then you can ask it to decode the instructions for you.
pub struct Chip8 {
    system: instruction_decoder::ChipSystem
}

impl Chip8 {
    ///Returns a new Chip8 system to use for emulation
    pub fn new() -> Self {
	return Chip8 {
	    system: instruction_decoder::ChipSystem::new()
	}
    }

    ///initializes the Chip8 with a program from a file.
    pub fn init(&mut self, file: File) {
	let file_buffer = BufReader::new(file)
	    .bytes()
	    .map(|x| match x {
		Ok(x) => x,
		Err(error) => panic!("error with reading program file: {}", error) })
	    .collect::<Vec<u8>>();

	self.system.load_program(file_buffer);
    }
}

impl timers::Timable for Chip8 {
    const ACTIONS_PER_SECOND: usize = 700;

    ///this implementation will cause the action (decoding an instruction) to be performed roughly 700 times per second.
    fn do_act(&mut self, current_moment: usize, second_size: usize) {
	if Self::now(current_moment, second_size) {
	    match self.system.decode_next_instruction() {
		Ok(_) => {},
		Err(error) => {
		    panic!("error with instruction decoding: {}", error);
		}
	    }
	}
    }
}
