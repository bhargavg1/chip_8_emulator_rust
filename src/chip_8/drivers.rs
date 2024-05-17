
//!implements various drivers for providing the chip 8 with sound and video.
//!The following are provided already, other drivers can be created to use different methods of drawing the screen.
//!You can do this by just implementing the VideoDriver and soundDriver trait onto your driver.

use std::io::Read;

use crate::chip_8::video::VideoDriver;
use crate::chip_8::timers::SoundDriver;
use crate::chip_8::keyboard::KeyboardDriver;

///Implements VideoDriver to draw the chip 8 display in the terminal.
///This driver is extremely rudimentary and simple, it just println's each line in the binary representation as it is stored.
///The VideoDriver trait has more info on how the display is stored.
pub struct TerminalNumbers;

impl VideoDriver for TerminalNumbers {
    fn draw(&self, bitmap: &[u64; 32]) {
	print!("{}[2J", 27 as char);
	println!("________________________________________________________________________");
	bitmap.iter().for_each(|x| println!("{:#066b}", x));
	println!("________________________________________________________________________");
    }
}

///Implements SoundDriver to make beeps for the chip 8. This just uses the standard terminal alarm ("\x07").
///This driver is extremely rudimentary and a hacky way of beeping.
pub struct TerminalBeep;

impl SoundDriver for TerminalBeep {
    fn set_beep(&self, state: bool) {
	if state {
	    print!("\x07");
	}
    }
}

pub struct KeySender {
    stdin: std::cell::RefCell<std::io::Stdin>
}

impl KeySender {
    pub fn new() -> Self {
	return KeySender {
	    stdin: std::cell::RefCell::new(std::io::stdin())
	}
    }
}

impl KeyboardDriver for KeySender {
    fn get_key_pressed(&self) -> Option<u8> {
	let mut read_chars = String::new();
	let size = self.stdin.borrow_mut().read_line(&mut read_chars).expect("failed to get keys: stdin read error");
	let read_chars = read_chars.as_bytes();
	if size > 1 {
	    return Some(read_chars[size - 2]);
	}
	return None;
    }
}
