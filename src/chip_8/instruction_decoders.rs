
//! this module implements the instruction decoder for the chip 8.
//! it also implements the various registers of the chip 8.

use std::fmt::Debug;
use crate::chip_8::{memory, timers, video, keyboard};

///Holds a list of closures which execute the decoded instruciton
///the closures are strategically ordered to match the chip instructions order,
/// the 0xxx instructions are first, 1xxx are second, fxxx are last, etc.
///you just insert that first hex number into the array to get the desired corresponding closure.
///with that closure, you just need to provide it with access to the ChipSystem and the instruction details,
/// so that the instruction can have the desired outcome.
const DECODED_INSTRUCTIONS: [fn(&mut ChipSystem, u16) -> Result<(), String>; 16] = [
    |system, input| { //instruction 0
	match input & 0x0FFF {
	    0x0E0 => {
		system.video.clear_buffer();
		system.video.update_screen();
	    }
	    0x0EE => {
		system.program_counter = system.stack.pop()?;
	    }
	    _ => return Err(format!("unimplemented usage of opcode {:#06x}", input))
	}
	return Ok(());
    },
    |system, input|  { //instruction 1
	system.program_counter = get_nnn(input);
	return Ok(());
    },
    |system, input| { //instruciton 2
	system.stack.push(system.program_counter)?;
	system.program_counter = get_nnn(input);
	return Ok(());
    },
    |system, input| { //instruction 3
	if system.registers.variable_register[get_x(input)] == get_nn(input) {
	    system.program_counter += 2;
	}
	return Ok(());
    },
    |system, input| { //instruction 4
	if system.registers.variable_register[get_x(input)] != get_nn(input) {
	    system.program_counter += 2;
	}
	return Ok(());
    },
    |system, input| { //instruciton 5
	match input & 0x000F {
	    0x0 => {
		let x = system.registers.variable_register[get_x(input)];
		let y = system.registers.variable_register[get_y(input)];
		if x == y {
		    system.program_counter += 2;
		}
	    },
	    _ => return Err(format!("unimplemented usage of opcode {:#06x}", input))
	}
	return Ok(());
    },
    |system, input| { //instruction 6
	system.registers.variable_register[get_x(input)] = get_nn(input);
	return Ok(());
    },
    |system, input| { //instruction 7
	let register_ref = &mut system.registers.variable_register[get_x(input)];
	(*register_ref) = (*register_ref).wrapping_add(get_nn(input));
	return Ok(());
    },
    |system, input| { //instruciton 8
	match get_n(input) {
	    0x0 => { //set x to y
		system.registers.variable_register[get_x(input)] = system.registers.variable_register[get_y(input)];
	    },
	    0x1 => { //x |= y
		system.registers.variable_register[get_x(input)] |= system.registers.variable_register[get_y(input)];
	    },
	    0x2 => { //x &= y
		system.registers.variable_register[get_x(input)] &= system.registers.variable_register[get_y(input)];
	    },
	    0x3 => { //x ^= y
		system.registers.variable_register[get_x(input)] ^= system.registers.variable_register[get_y(input)];
	    },
	    0x4 => { //x += y, sets vf to 1 if overflow
		let x = system.registers.variable_register[get_x(input)] as usize;
		let y = system.registers.variable_register[get_y(input)] as usize;
		system.registers.variable_register[get_x(input)] = ((x + y) & 0xFF) as u8;
		system.registers.variable_register[0xF] = ((x + y) >> 8) as u8;
	    }
	    0x5 => { //x -= y, sets vf to 1 if didnt borrow
		let x = system.registers.variable_register[get_x(input)];
		let y = system.registers.variable_register[get_y(input)];
		system.registers.variable_register[get_x(input)] = x.wrapping_sub(y);
		system.registers.variable_register[0xF] = if x > y {1} else {0};
	    },
	    0x6 => { //x = (y >> 1), set vf to y & 0x1
		system.registers.variable_register[get_x(input)] = system.registers.variable_register[get_y(input)] >> 1;
		system.registers.variable_register[0xF] = system.registers.variable_register[get_y(input)] & 0x01;
	    }
	    0xE => { //x = (y << 1), set vf to y & 0x80
		system.registers.variable_register[get_x(input)] = system.registers.variable_register[get_y(input)] << 1;
		system.registers.variable_register[0xF] = system.registers.variable_register[get_y(input)] & 0x80;
	    }
	    _ => return Err(format!("unimplemented usage of opcode {:#06x}", input))
	}
	return Ok(());
    },
    |system, input| { //instruciton 9
	match get_n(input) {
	    0x0 => {
		let x = system.registers.variable_register[get_x(input)];
		let y = system.registers.variable_register[get_y(input)];
		if x != y {
		    system.program_counter += 2;
		}
	    },
	    _ => return Err(format!("unimplemented usage of opcode {:#06x}", input))
	}
	return Ok(());
    },
    |system, input| { //instruction A
	system.registers.index_register = input & 0x0FFF;
	return Ok(());
    },
    |system, input| { //instruciton B
	system.program_counter = get_nnn(input) + system.registers.variable_register[0] as u16;
	return Ok(());
    },
    |system, input| { //instruciton C
	let random_allocation = 0u8;
	let random_address = random_allocation as *const u8 as usize;
	system.registers.variable_register[get_x(input)] = (((random_address / std::mem::size_of::<usize>()) & 0xFF) as u8) & get_nn(input);
	return Ok(());
    },
    |system, input| { //instruciton D
	let vx = system.registers.variable_register[get_x(input)];
	let vy = system.registers.variable_register[get_y(input)];
	let mut height = get_n(input);
	let i = system.registers.index_register as usize;
	let sprite_lines = &system.ram.memory_array[i..(i + height)] as *const [u8];
	system.video.draw_sprite(vx, vy, Box::new(move || {
	    if height > 0 {
		height -= 1;
		unsafe {
		    return Some(((*sprite_lines)[(height) as usize], (height) as u8));
		}
	    }
	    return None;
	}));
	system.video.update_screen();
	return Ok(());
    },
    |system, input| { //instruciton E
	let skip_if_equal = match get_nn(input) {
	    0x9E => true,
	    0xA1 => false,
	    _ => return Err(format!("unimplemented usage of opcode {:#06x}", input))
	};
	match system.keyboard.which_key_pressed() {
	    Some(key) => {
		if (key == system.registers.variable_register[get_x(input)]) == skip_if_equal {
		    system.program_counter += 2;
		}
	    },
	    None => {
		system.program_counter += 2
	    }
	}
	return Ok(());
    },
    |system, input| { //instruciton F
	match get_nn(input) {
	    0x07 => system.registers.variable_register[get_x(input)] = system.delay_timer.time_value,
	    0x15 => system.delay_timer.time_value = system.registers.variable_register[get_x(input)],
	    0x18 => system.sound_timer.time_value = system.registers.variable_register[get_x(input)],
	    0x1E => {
		system.registers.index_register += system.registers.variable_register[get_x(input)] as u16;
		if system.registers.index_register & 0x1000 == 0x1000 {
		    system.registers.variable_register[0xF] = 1;
		    system.registers.index_register &= 0x0FFF;
		}
	    },
	    0x0A => {
		loop {
		    match system.keyboard.which_key_pressed() {
			Some(key) => {
			    system.registers.variable_register[get_x(input)] = key;
			    break;
			},
			None => {}
		    }
		}
	    },
	    0x29 => {
		system.registers.index_register = system.ram.get_character(system.registers.variable_register[get_x(input)] as usize);
	    },
	    0x33 => {
		let number = system.registers.variable_register[get_x(input)];
		let i = system.registers.index_register as usize;
		system.ram.memory_array[i] = number / 100;
		system.ram.memory_array[i + 1] = (number / 10) % 10;
		system.ram.memory_array[i + 2] = number % 10;		    
	    },
	    0x55 => {
		let i = system.registers.index_register as usize;
		let final_register = get_x(input);
		for x in 0..(final_register + 1) {
		    system.ram.memory_array[i + x] = system.registers.variable_register[x];
		}
	    },
	    0x65 => {
		let i = system.registers.index_register as usize;
		let final_register = get_x(input);
		for x in 0..(final_register + 1) {
		    system.registers.variable_register[x] = system.ram.memory_array[i + x];
		}
	    }
	    _ => return Err(format!("unimplemented usage of opcode {:#06x}", input))
	}
	return Ok(());
    }
];

///Contains all the components nessecary to run a chip 8 program
///the instructions work on this struct and modify it's state.
pub struct ChipSystem <'a> {
    program_counter: u16,
    registers: memory::RegisterSet,
    stack: memory::Stack,
    ram: memory::EntireMemory,
    video: video::VideoDisplay<'a>,
    sound_timer: timers::SoundTimer<'a>,
    delay_timer: timers::DelayTimer,
    keyboard: keyboard::Keyboard<'a>
}

impl <'a> ChipSystem <'a> {
    ///returns a new uninitialized ChipSystem. You will have to provide the program after the initialization
    ///see mod.rs TimedRunner struct for the init() function it has.
    pub fn new<T, U, V>(video_driver: T, sound_driver: U, keyboard_driver: V) -> Self  where
	T: video::VideoDriver + 'a,
	U: timers::SoundDriver + 'a,
	V: keyboard::KeyboardDriver + 'a {
	
	return ChipSystem {
	    program_counter: 0,
	    registers: memory::RegisterSet::new(),
	    stack: memory::Stack::new(),
	    ram: memory::EntireMemory::new(),
	    video: video::VideoDisplay::new(video_driver),
	    sound_timer: timers::SoundTimer::new(sound_driver),
	    delay_timer: timers::DelayTimer::new(),
	    keyboard: keyboard::Keyboard::new(keyboard_driver)
	}
    }
}

impl Debug for ChipSystem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	return f.debug_struct("Internals: ")
	    .field("program counter", &self.program_counter)
	    .field("index", &self.registers.index_register)
	    .finish();
    }
}

///This takes a vector of bytes which make up the program and pushes them into memory.
///It is never called directly, see init() in mod.rs.
pub fn load_program_from_vector(system: &mut ChipSystem, program_array: Vec<u8>) {
    system.program_counter = system.ram.load_program(program_array);
}

///decodes the next instruction at the program_counter.
///also ticks the timer when needed.
pub fn decode_next_instruction(system: &mut ChipSystem) -> Result<(), String> {
    let instruction_first_byte = system.ram.memory_array[system.program_counter as usize] as u16;
    let instruction_second_byte = system.ram.memory_array[(system.program_counter + 1) as usize] as u16;
    let combined_instruction = (instruction_first_byte << 8) + instruction_second_byte;
    system.program_counter += 2;
    return DECODED_INSTRUCTIONS[get_instruction_category(combined_instruction)](system, combined_instruction);
}

///Takes the chipsystem timers and decrements them exactly once if they have a value above 0.
pub fn tick_timers(system: &mut ChipSystem) {
    system.sound_timer.tick_down();
    system.delay_timer.tick_down();
}

///takes an input instrucition and returns the second nibble from the left.
///this second nibble usually represents a variable register (there are 16 of them so 0-F to choose one of them).
const fn get_x(input: u16) -> usize {
    return ((input & 0x0F00) >> 8) as usize;
}

///takes the input instrucition and returns the third nibble from the left.
///this third nibble usually represents a variable register too, though the use of this
/// nibble as a vr is rare (used in the display dxyn instruction for one).
const fn get_y(input: u16) -> usize {
    return ((input & 0x00F0) >> 4) as usize;
}

///takes an input instruction and returns the last nibble from the left
const fn get_n(input: u16) -> usize {
    return (input & 0x000F) as usize;
}

///takes an input instruction and returns the last two nibbles (second byte)
const fn get_nn(input: u16) -> u8 {
    return (input & 0x00FF) as u8;
}

///takes and input instruction and returns the last three nibbles.
const fn get_nnn(input: u16) -> u16 {
    return input & 0x0FFF;
}

///takes an input instruction and returns the very first nibble
///this is used to index the closures array to get the desired closure which executes the desired instruction.
const fn get_instruction_category(input: u16) -> usize {
    return ((input & 0xF000) >> 12) as usize;
}
