
//! This module deals with the display for the chip 8
//!

///Defines how a driver should work to draw the display of the chip 8.
///many drivers can be implemented, so the chip 8 can make use of many methods, gui or cli.
pub trait VideoDriver {
    fn draw(&self, bitmap: &[u64; 32]);
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
	    buffer: [0u64; 32],
	    driver
	};
    }

    ///clears the screen, making everything empty
    pub fn clear_buffer(&mut self) {
	self.buffer.iter_mut().for_each(|x| *x = 0u64);
    }

    ///Draws a sprite onto the screen at a given x and y coordinate.
    ///The sprite (get_line) is a closure which works similarly to an iterator. Every time the get_line closure is called, the next
    ///byte of the sprite is returned. Sprite bytes will be drawn until None are left.
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

impl std::fmt::Debug for VideoDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	return f.debug_struct("VideoDisplay")
	    .field("buffer", &self.buffer)
	    .finish();
    }
}
