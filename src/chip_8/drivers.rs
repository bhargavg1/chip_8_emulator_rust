
//!implements various drivers for providing the chip 8 with sound, video, and keyboard.
//!The following are provided already, other drivers can be created to use different methods of drawing the screen.
//!You can do this by just implementing the VideoDriver, SoundDriver, and KeyboardDriver traits onto your driver, then supplying them to the chip.

use crate::chip_8::video::VideoDriver;
use crate::chip_8::timers::SoundDriver;
use crate::chip_8::keyboard::KeyboardDriver;

///Implements VideoDriver to draw the chip 8 display in the terminal.
///This driver is extremely rudimentary and simple, it just println's each line in the binary representation as it is stored.
///The VideoDriver trait has more info on how the display is stored.
pub struct StdoutDisplay {
    display_array: [char; 4128]
}

impl StdoutDisplay {
    pub fn new() -> Self {
	let mut termsize = libc::winsize { //the terminal state will be stored in here.
	    ws_row: 0,
	    ws_col: 0,
	    ws_xpixel: 0,
	    ws_ypixel: 0
	};
	unsafe {
	    libc::ioctl(1, libc::TIOCGWINSZ, &mut termsize as *const _ as *const libc::c_void); //gets the terminal current state.
	}
	if termsize.ws_row < 32 {
	    panic!("Your terminal is not tall enough: needed 32 lines, only got {}\n try fullscreening terminal.", termsize.ws_row);
	}
	if termsize.ws_col < 128 {
	    panic!("Your terminal is not wide enough: needed 128 lines, only got {}\n try putting it in fullscreen", termsize.ws_col);
	}
	return StdoutDisplay {
	    display_array: ['\u{2591}'; 4128]
	}
    }
}

impl VideoDriver for StdoutDisplay {
    fn draw(&mut self, bitmap: &[u64; 32]) {
	bitmap.iter().enumerate().for_each(|(i, val)| {
	    for offset in 0..64 {
		if *val & (0x1u64 << (63 - offset)) != 0 {
		    self.display_array[(offset * 2) + (129 * i)] = '\u{2588}';
		    self.display_array[(offset * 2) + (129 * i) + 1] = '\u{2588}';
		} else {
		    self.display_array[(offset * 2) + (129 * i)] = '\u{2591}';
		    self.display_array[(offset * 2) + (129 * i) + 1] = '\u{2591}';
		}
	    }
	    self.display_array[(129 * i) + 128] = '\n';
	});
	print!("\x1Bc{}",self.display_array.iter().collect::<String>());
    }
}

///Implements SoundDriver to make beeps for the chip 8. This just uses the standard terminal alarm ("\x07").
///This driver is extremely rudimentary and a hacky way of beeping.
pub struct TerminalBeep;

impl TerminalBeep {
    pub fn new() -> Self {
	return TerminalBeep;
    }
}

impl SoundDriver for TerminalBeep {
    fn set_beep(&self, state: bool) {
	if state {
	    print!("\x07");
	}
    }
}

///implements the KeyboardDriver to send key presses to the chip8. This is a very rudimentary driver also.
///you press one of keys (1234,qwer,asdf,zxvc make up the 4x4 keypad).
///Like the SoundDriver implementation above, this is a hacky way of getting keyboard input I think, but I didnt want to use
/// some well-made library and bring in all these dependencies, when I could try to make this myself and learn a bit about stdin and stdout.
///This driver uses libc functions, namely the tcsetattr() to disable the terminal canonical mode. This causes the stdin to be basically unbuffered,
/// allowing the program to instantly read a keypress the moment you press it. Without it, you would have to press enter after every keystroke to
/// put a newline in the stdin for the program to recieve the input.
pub struct StdinKeysender {
    _cleanerthread: std::thread::JoinHandle<()>,
    current_pressed_reader: std::sync::Arc<std::sync::Mutex<Option<u8>>>
}

impl StdinKeysender {
    ///this version of the driver requires that you call the new() function, you cant generate an instance yourself.
    ///a new thread is spawned whose only purpose is to read the most recent key on the keypad that is pressed, and
    /// then set that as the current key (current_pressed_reader).
    pub fn new() -> Self {
	let mut termsettings = libc::termios { //the numbers here are just placeholders, none of them will actually be used.
	    c_iflag: 0,
	    c_oflag: 0,
	    c_cflag: 0,
	    c_lflag: 0,
	    c_line: 0,
	    c_cc: [0; 32],
	    c_ispeed: 0,
	    c_ospeed: 0
	};
	unsafe {
	    libc::tcgetattr(0, &mut termsettings as *mut libc::termios); //we get the current stdin configuration and store it in termsettings.
	    termsettings.c_lflag &= u32::MAX ^ libc::ICANON; //we keep all settings same except ICANON, we dont want canonical mode, we want instant input.
	    termsettings.c_lflag &= u32::MAX ^ libc::ECHO; //we also dont need to echo all of our keystrokes back into the terminal.
	    libc::tcsetattr(0, libc::TCSANOW, &mut termsettings as *mut libc::termios); //we then apply the modified settings back on stdin (fd 0).
	}
	let current_pressed_reader = std::sync::Arc::new(std::sync::Mutex::new(None)); //the current key that is being pressed is in here
	let current_pressed = current_pressed_reader.clone(); //the new thread below uses this to communicate with the main thread about most recent keys.
	return StdinKeysender {
	    _cleanerthread: std::thread::spawn(move || {
		loop {
		    std::thread::sleep(std::time::Duration::from_millis(10));
		    let mut new_key = current_pressed.lock().expect("unable to block this thread");
		    let mut readbuffer = [0u8; 1];
		    unsafe {		    
			libc::read(0, &mut readbuffer as *mut _ as *mut libc::c_void, 1);
		    }
		    (*new_key) = match readbuffer[0] {
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
		    };
		}
	    }),
	    current_pressed_reader
	}
    }
}

impl KeyboardDriver for StdinKeysender {
    fn get_key_pressed(&mut self) -> Option<u8> {
	return self.current_pressed_reader.lock().expect("unable to block the cleaner thread").take();
    }
}
