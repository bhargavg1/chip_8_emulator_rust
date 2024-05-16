
//!implements various drivers for providing the chip 8 with sound and video.

use crate::chip_8::video::VideoDriver;
use crate::chip_8::timers::SoundDriver;

///Implements VideoDriver to draw the chip 8 display in the terminal.
pub struct TerminalNumbers;

impl TerminalNumbers {
    ///Returns a new TerminalDriver object to be used by the VideoDisplay.
    pub fn new() -> Box<Self> {
	return Box::new(Self);
    }
}

impl VideoDriver for TerminalNumbers {
    fn draw(&self, bitmap: &[u64; 32]) {
	print!("{}[2J", 27 as char);
	println!("________________________________________________________________________");
	bitmap.iter().for_each(|x| println!("{:#066b}", x));
	println!("________________________________________________________________________");
    }
}

pub struct TerminalBeep;

impl TerminalBeep {
    pub fn new() -> Box<Self> {
	return Box::new(Self);
    }
}

impl SoundDriver for TerminalBeep {
    fn set_beep(&self, state: bool) {
	if state {
	    print!("\x07");
	}
    }
}
