
//! contains the various modules which implement the features of the chip 8.

mod memory;
mod video;
mod timers;
mod instruction_decoders;
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
    system: instruction_decoders::ChipSystem<'a>,
    time_since_timer_decrement: u64,
    time_since_last_decode: u64
}

impl <'a> TimedRunner <'a> {
    pub fn new<T, U, V>(video_driver: T, sound_driver: U, keyboard_driver: V) -> Self where
	T: VideoDriver + 'a,
	U: SoundDriver + 'a,
	V: KeyboardDriver + 'a {
	
	return TimedRunner {
	    system: instruction_decoders::ChipSystem::new(video_driver, sound_driver, keyboard_driver),
	    time_since_timer_decrement: 0,
	    time_since_last_decode: 0
	};
    }
    
    fn tick_chip(&mut self) {
	self.time_since_timer_decrement += MICROSECONDS_PER_TICK;
	self.time_since_last_decode += MICROSECONDS_PER_TICK;
	
	if self.time_since_timer_decrement > MICROSECONDS_PER_TIMER_DECREMENT {
	    instruction_decoders::tick_timers(&mut self.system);
	    self.time_since_timer_decrement = 0;
	}
	if self.time_since_last_decode > MICROSECONDS_PER_INSTRUCTION_DECODE {
	    match instruction_decoders::decode_next_instruction(&mut self.system) {
		Ok(_) => {},
		Err(error) => panic!("error decoding instruction: {}", error)
	    }
	    self.time_since_last_decode = 0;
	}
    }

    pub fn decode_next_immediately(&mut self) {
	while self.time_since_timer_decrement != 0 {
	    self.tick_chip();
	}
    }

    pub fn decode_next_timed(&mut self, speed_multiplier: f64) {
	self.tick_chip();
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

	instruction_decoders::load_program_from_vector(&mut self.system, file_buffer);
    }

}
