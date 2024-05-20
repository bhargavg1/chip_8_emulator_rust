
//!implements various drivers for providing the chip 8 with sound, video, and keyboard.
//!The following are provided already, other drivers can be created to use different methods of drawing the screen.
//!You can do this by just implementing the VideoDriver, SoundDriver, and KeyboardDriver traits onto your driver, then supplying them to the chip.

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

///implements the KeyboardDriver to send key presses to the chip8. This is a very rudimentary driver only for testing purposes.
///you press one of keys (1234,qwer,asdf,zxvc make up the 4x4 keypad), and then you have to press enter for the chip8 to register that key.
pub struct KeySender {
    stdin: std::cell::RefCell<std::io::Stdin>
}

impl KeySender {
    ///this version of the driver requires that you call the new() function, you cant generate an instance yourself.
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
	    return match read_chars[size - 2] {
		b'1' => Some(0x1),
		b'2' => Some(0x2),
		b'3' => Some(0x3),
		b'4' => Some(0xC),
		b'q' => Some(0x4),
		b'w' => Some(0x5),
		b'e' => Some(0x6),
		b'r' => Some(0xD),
		b'a' => Some(0x7),
		b's' => Some(0x8),
		b'd' => Some(0x9),
		b'f' => Some(0xE),
		b'z' => Some(0xA),
		b'x' => Some(0x0),
		b'c' => Some(0xB),
		b'v' => Some(0xF),
		_ => None
	    }
	}
	return None;
    }
}
