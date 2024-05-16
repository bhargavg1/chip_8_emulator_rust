
//! This module deals with the display for the chip 8
//!

///This struct holds the video display. It uses 32 u64 integers to hold the 64 x 32 frame.
pub struct VideoDisplay{
    bitmap: [u64; 32]
}

impl VideoDisplay {
    pub fn new() -> Self {
	return VideoDisplay { bitmap: [0xAAAAAAAAAAAAAAAAu64; 32] };
    }
}


