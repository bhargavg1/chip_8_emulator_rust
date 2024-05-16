
//! contains the various modules which implement the features of the chip 8.

mod memory;
mod video;
mod timers;
mod instruction_decoder;

pub use self::instruction_decoder::ChipSystem;

use std::fs::File;
use std::io::{BufReader, Read};

///initializes the Chip8 with a program from a file.
pub fn init(system: &mut ChipSystem, file: File) {
    let file_buffer = BufReader::new(file)
	.bytes()
	.map(|x| match x {
	    Ok(x) => x,
	    Err(error) => panic!("error with reading program file: {}", error) })
	.collect::<Vec<u8>>();

    system.load_program(file_buffer);
}

///call this function to properly run the chip 8 at the correct speeds
///it takes care of the instruction decode timing and the ticking of the sound and delay timer.
///just insert 1 for normal speed (700hz instruction decoding, 60hz timer tick), or a smaller or bigger number for changing the speed.
pub fn timed_tick(system: &mut ChipSystem, speed_multiplier: f64) {
    let timimg_lcm: u32 = 700 * 60;
}
