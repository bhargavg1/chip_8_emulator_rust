
//! contains the various modules which implement the features of the chip 8.

mod memory;
mod video;
mod timers;
mod instruction_decoder;
mod keyboard;
pub mod drivers;

pub use video::VideoDriver;
pub use timers::SoundDriver;
pub use keyboard::KeyboardDriver;

use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Duration;
use std::thread;

const TIMING_LCM: u64 = 700 * 60;
const MICROSECONDS_PER_TICK: u64 = 1000000 / TIMING_LCM;
const MICROSECONDS_PER_TIMER_DECREMENT: u64 = MICROSECONDS_PER_TICK * 700;
const MICROSECONDS_PER_INSTRUCTION_DECODE: u64 = MICROSECONDS_PER_TICK * 60;

pub struct TimedRunner <'a> {
    system: instruction_decoder::ChipSystem<'a>,
    time_since_timer_decrement: u64,
    time_since_last_decode: u64
}

impl <'a> TimedRunner <'a> {
    pub fn new<T, U, V>(video_driver: T, sound_driver: U, keyboard_driver: V) -> Self where
	T: VideoDriver + 'a,
	U: SoundDriver + 'a,
	V: KeyboardDriver + 'a {
	
	return TimedRunner {
	    system: instruction_decoder::ChipSystem::new(video_driver, sound_driver, keyboard_driver),
	    time_since_timer_decrement: 0,
	    time_since_last_decode: 0
	};
    }
    
    fn decode_instructions(&mut self) {
	if MICROSECONDS_PER_INSTRUCTION_DECODE < self.time_since_last_decode {
	    match self.system.decode_next_instruction() {
		Ok(_) => {},
		Err(error) => panic!("Error executing instruction: {}", error)
	    }
	    self.time_since_last_decode = 0;
	}

	if MICROSECONDS_PER_TIMER_DECREMENT < self.time_since_timer_decrement {
	    self.system.tick_timers();
	    self.time_since_timer_decrement = 0;
	}
	self.time_since_last_decode += MICROSECONDS_PER_TICK;
	self.time_since_timer_decrement += MICROSECONDS_PER_TICK
    }

    pub fn decode_next_immediately(&mut self) {
	for _ in 0..MICROSECONDS_PER_INSTRUCTION_DECODE {
	    self.decode_instructions();
	}
    }

    pub fn decode_next_timed(&mut self, speed_multiplier: f64) {
	self.decode_instructions();
	thread::sleep(Duration::from_micros((MICROSECONDS_PER_TICK as f64 * speed_multiplier) as u64));
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
