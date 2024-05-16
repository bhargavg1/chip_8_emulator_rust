
//! This module deals with the display for the chip 8
//! 

use sdl2::{
    event::Event, keyboard::Keycode, libc::SYS_gettimeofday, pixels::Color, render::Canvas
};

///Defines how a driver should work to draw the display of the chip 8.
pub trait VideoDriver {
    fn draw(&self, bitmap: &[u64; 32]);
}

///Implements VideoDriver to draw the chip 8 display in the terminal.
pub struct TerminalDriver;

impl TerminalDriver {
    ///Returns a new TerminalDriver object to be used by the VideoDisplay.
    pub fn new() -> Box<Self> {
	return Box::new(TerminalDriver);
    }
}

impl VideoDriver for TerminalDriver {
    fn draw(&self, bitmap: &[u64; 32]) {
	bitmap.iter().for_each(|x| println!("{:#066b}", x));
    }
}

///This struct holds the video display. It uses 32 u64 integers to hold the 64 x 32 frame.
pub struct VideoDisplay{
    buffer: [u64; 32],
    driver: Box<dyn VideoDriver>
}

impl VideoDisplay {
    ///Returns a new video display, with black and white horizontal lines
    ///You need to supply the driver that the display will use. The current options are a termianl display and
    /// a graphical display that uses SDL2.
    pub fn new(driver: Box<dyn VideoDriver>) -> Self {
	return VideoDisplay {
	    buffer: [0xAAAAAAAAAAAAAAAAu64; 32],
	    driver
	};
    }

    ///clears the screen, making everything empty
    pub fn clear_buffer(&mut self) {
	self.buffer.iter_mut().for_each(|x| *x = 0u64);
    }

    pub fn draw_sprite(&mut self, inputx: u8, inputy: u8, get_line: Box<dyn Fn() -> Option<(u8, u8)>>) {
	while let Some((sprite_line, y_offset)) = get_line() {
	    self.buffer[(inputy.wrapping_add(y_offset) % 32) as usize] ^= (((sprite_line as u64) << 56) >> (inputx % 64)) as u64;
	}
    }

    ///updates the screen with the current latest buffer that is stored.
    ///uses the stored VideoDriver in order to accomplish the graphics.
    pub fn update_screen(&self) {
	self.driver.draw(&self.buffer);
    }
}
