
//! This is a module to deal with emulating the RAM of the chip8
//! A struct with implemented funcitons is provided to help with this.

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
struct EntireMemory {
    memory_array: [u8; 4096] //the full 4 kilobytes of memory is stored in a single array.
}

/// This defines the methods for the Entirememory
impl EntireMemory {
    ///This function will generate a new ram setup for the chip8 with the default font already loaded
    pub fn new_with_normal_font() -> Self {
	let mut new_memory = EntireMemory { memory_array: [0u8; 4096] };
	new_memory.apply_font(&CHIP_FONT);
	
	return new_memory;
    }

    ///This function will apply a font to the memory
    pub fn apply_font(&mut self, font: &[u8; 80]) {
	let mut chip_font_iter = font.iter();
	self.memory_array.iter_mut().for_each(|x| *x = *chip_font_iter.next().expect("ran out of font characters"));
    }
}
