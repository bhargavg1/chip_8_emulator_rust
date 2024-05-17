
//! This module deals with the display for the chip 8
//! It contains the VideoDisplay structure and various other methods to draw onto the screen
//! This module though does not contain the methods to actually show the display though, a VideoDriver must be implemented and
//!  supplied to the VideoDisplay so that it can show you the display.
//! Some drivers are already implemented in the chip8::drivers module for use by the display.

///Defines how a driver should work to draw the display of the chip 8.
///many drivers can be implemented, so the chip 8 can make use of many methods, gui or cli.
///
///In this implementation of the chip 8 video, the display is stored as a an array of 32 u64 numbers, to represent
/// the 64x32 screen. Therefore, each pixel just gets one bit.
///In order to implement a VideoDriver, you just
pub trait VideoDriver {
    fn draw(&self, bitmap: &[u64; 32]);
}

///This struct holds the video display. It uses 32 u64 integers to hold the 64 x 32 frame.
///Like described in the VideoDriver description, the screen is stored as an array of integers, each integer is a u64 (it has 64 bits),
/// and there are 32 of them in the array.
///You must provide a VideoDriver though in order to actually be able to see the display.
pub struct VideoDisplay{
    buffer: [u64; 32],
    driver: Box<dyn VideoDriver>
}

impl VideoDisplay {
    ///Returns a new VideoDisplay, which is set to completely blank. You need to supply the driver that the display will use. 
    pub fn new(driver: Box<dyn VideoDriver>) -> Self {
	return VideoDisplay {
	    buffer: [0u64; 32],
	    driver
	};
    }

    ///clears the screen, making everything empty
    pub fn clear_buffer(&mut self) {
	self.buffer.iter_mut().for_each(|x| *x = 0u64);
    }

    ///Draws a sprite onto the screen at a given x and y coordinate.
    ///The sprite (get_line) is a closure which works similarly to rust's Iterator object. Every time the get_line closure is called, the next
    ///byte of the sprite is returned (each byte is the binary representation of a row of pixels). Sprite bytes will be drawn until None are left.
    pub fn draw_sprite(&mut self, inputx: u8, inputy: u8, mut get_line: Box<dyn FnMut() -> Option<(u8, u8)>>) {
	while let Some((sprite_line, y_offset)) = get_line() {
	    self.buffer[(inputy.wrapping_add(y_offset) % 32) as usize] ^= ((sprite_line as u64) << 56) >> (inputx % 64);
	}
    }

    ///updates the screen with the current latest buffer that is stored.
    ///uses the stored VideoDriver in order to accomplish the graphics.
    pub fn update_screen(&self) {
	self.driver.draw(&self.buffer);
    }
}
