
//! This is the keyboard

pub trait KeyboardDriver {
    fn get_key_pressed(&self) -> Option<u8>;
}


pub struct Keyboard <'a> {
    keyboard_driver: Box<dyn KeyboardDriver + 'a>
}

impl <'a> Keyboard <'a> {
    pub fn new<T: KeyboardDriver + 'a>(keyboard_driver: T) -> Self{
	return Keyboard {
	    keyboard_driver: Box::new(keyboard_driver)
	}
    }

    pub fn which_key_pressed(&self) -> Option<u8> {
	return self.keyboard_driver.get_key_pressed();
    }
}

