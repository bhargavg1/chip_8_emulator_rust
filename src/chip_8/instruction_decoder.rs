
//! this module implements the instruction decoder for the chip 8.
//! it also implements the various registers of the chip 8.

use crate::chip_8::{memory, timers, video, keyboard};

///Holds a list of closures which execute the decoded instruciton
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
	return Err(format!("unimplemented usage of opcode {:#06x}", input));
    },
    |system, input| { //instruciton C
	let random_allocation = 0u8;
	let random_address = random_allocation as *const u8 as usize;
	system.registers.variable_register[get_x(input)] = (((random_address / std::mem::size_of::<usize>()) & 0xFF) as u8) & get_nn(input);
	return Err(format!("unimplemented usage of opcode {:#06x}", input));
    },
    |system, input| { //instruciton D
	let vx = system.registers.variable_register[get_x(input)];
	let vy = system.registers.variable_register[get_y(input)];
	let mut height = get_n(input);
	let i = system.registers.index_register as usize;
	let sprite_lines = &system.ram.memory_array[i..(i as usize + height)] as *const [u8];
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
	return Err(format!("unimplemented usage of opcode {:#06x}", input));
    },
    |system, input| { //instruciton F
	return Err(format!("unimplemented usage of opcode {:#06x}", input));
    }
];

///Contains all the components nessecary to run a chip 8 program
pub struct ChipSystem <'a> {
    program_counter: u16,
    registers: memory::RegisterSet,
    stack: memory::Stack,
    ram: memory::EntireMemory,
    video: video::VideoDisplay<'a>,
    sound_timer: timers::DelayTimer,
    delay_timer: timers::SoundTimer<'a>,
    keyboard: keyboard::Keyboard<'a>
}

impl <'a> ChipSystem <'a> {
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
	    sound_timer: timers::DelayTimer::new(),
	    delay_timer: timers::SoundTimer::new(sound_driver),
	    keyboard: keyboard::Keyboard::new(keyboard_driver)
	}
    }

    pub fn load_program(&mut self, program_array: Vec<u8>) {
	self.program_counter = self.ram.load_program(program_array);
    }

    ///decodes the next instruction at the program_counter.
    ///also ticks the timer when needed.
    pub fn decode_next_instruction(&mut self) -> Result<(), String> {
	let instruction_first_byte = self.ram.memory_array[self.program_counter as usize] as u16;
	let instruction_second_byte = self.ram.memory_array[(self.program_counter + 1) as usize] as u16;
	let combined_instruction = (instruction_first_byte << 8) + instruction_second_byte;
	self.program_counter += 2;
	return DECODED_INSTRUCTIONS[get_instruction_category(combined_instruction)](self, combined_instruction);
    }

    pub fn tick_timers(&mut self) {
	self.sound_timer.tick_down();
	self.delay_timer.tick_down();
    }
}

const fn get_x(input: u16) -> usize {
    return ((input & 0x0F00) >> 8) as usize;
}

const fn get_y(input: u16) -> usize {
    return ((input & 0x00F0) >> 4) as usize;
}

const fn get_n(input: u16) -> usize {
    return (input & 0x000F) as usize;
}

const fn get_nn(input: u16) -> u8 {
    return (input & 0x00FF) as u8;
}

const fn get_nnn(input: u16) -> u16 {
    return input & 0x0FFF;
}

const fn get_instruction_category(input: u16) -> usize {
    return ((input & 0xF000) >> 12) as usize;
}
