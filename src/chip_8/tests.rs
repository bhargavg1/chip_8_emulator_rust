
use crate::chip_8::{instruction_decoder, memory, timers, video};

use super::video::TerminalDriver;

#[test]
fn test_video() {
    let mut chip_video = video::VideoDisplay::new(TerminalDriver::new());
    chip_video.update_screen();
    chip_video.clear_buffer();
    chip_video.update_screen();
}
