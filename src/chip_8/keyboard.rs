
//! This is the keyboard for the chip8.
//! it provides a struct with the basic functions of a keyboard, and a KeyboardDriver trait to implement a keyboard.

///this trait is used to interface with different implementations of the chip8 keyboard.
///see the drivers.rs file for a simple implentation.
pub trait KeyboardDriver {
    fn get_key_pressed(&self) -> Option<u8>;
}

///this is a keyboard object which used a KeyboardDriver to get and interpret keypresses.
pub struct Keyboard <'a> {
    keyboard_driver: Box<dyn KeyboardDriver + 'a>
}

impl <'a> Keyboard <'a> {
    ///returns a new keyboard which uses the desired keyboard.
    pub fn new<T: KeyboardDriver + 'a>(keyboard_driver: T) -> Self{
	return Keyboard {
	    keyboard_driver: Box::new(keyboard_driver)
	}
    }

    ///tells you which key is pressed.
    pub fn which_key_pressed(&self) -> Option<u8> {
	return self.keyboard_driver.get_key_pressed();
    }
}

