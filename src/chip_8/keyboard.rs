
//! This is the keyboard


pub trait KeyboardDriver {
    fn get_key_pressed(&self) -> Option<u8>;
}


pub struct Keyboard {
    keyboard_driver: Box<dyn KeyboardDisplay>
}

impl Keyboard {
    pub fn new() -> Self {
	
    }
}
