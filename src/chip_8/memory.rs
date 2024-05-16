
//! This is a module to deal with emulating the RAM of the chip8
//! A Stack struct and and EntireMemory struct are provided to deal with these two components

///This array contains a default font for the chip 8.
const CHIP_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

///This struct takes care of the RAM for the chip8
pub struct EntireMemory {
    pub memory_array: [u8; 4096], //the full 4 kilobytes of memory is stored in a single array.
    font_beginning_index: u16,
}

/// This defines the methods for the Entirememory
impl EntireMemory {
    ///This function will generate a new ram setup for the chip8 with the default font already loaded
    pub fn new() -> Self {
	let mut new_memory = EntireMemory {
	    memory_array: [0u8; 4096],
	    font_beginning_index: 0,
	};
	new_memory.apply_font(&CHIP_FONT);
	
	return new_memory;
    }

    ///This function will apply a font to the memory
    pub fn apply_font(&mut self, font: &[u8; 80]) {
	self.memory_array[0..79]
	    .iter_mut()
	    .enumerate()
	    .for_each(|(i, val)| *val = font[i]);
    }
}

///This stack comes with 64 bytes of space, and can store up to 32 addresses (the addresses are 16 bit each).
///Each stack frame can only store a 12 bit number, for representing a memory address.
pub struct Stack {
    stack_array: [u16; 32],
    stack_size: u16,
    stack_position: usize,
}

impl Stack {
    ///Returns a new stack object.
    pub fn new() -> Self {
	return Stack {
	    stack_array: [0u16; 32],
	    stack_size: 32,
	    stack_position: 0
	};
    }

    ///This method will push an address onto the stack, the address can only be 12 bits long maximum (max number 4096).
    ///If there is no more space on the stack, then an Err() is returned.
    pub fn push(&mut self, value: u16) -> Result<(), ()> {
	if value > 4096 || self.stack_position == 64 {
	    return Err(());
	} else {
	    self.stack_array[self.stack_position] = value;
	    return Ok(());
	}
    }

    ///pops an address from the stack.
    ///If there is no more things to be popped, then an Err() is returned.
    pub fn pop(&mut self) -> Result<u16, ()> {
	if self.stack_position == 0 {
	    return Err(());
	} else {
	    self.stack_position -= 1;
	    return Ok(self.stack_array[self.stack_position]);
	}
    }
}

///This implements the registers for the chip 8.
///It includes the index register (I) and the 16 one-byte variable registers (V0 - VF).
pub struct RegisterSet {
    pub index_register: u16,
    pub variable_register: [u8; 16]
}
