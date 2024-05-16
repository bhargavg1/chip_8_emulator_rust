
//! this module implements the instruction decoder for the chip 8.
//! it also implements the various registers of the chip 8.

use crate::chip_8::{memory, timers, video};

///Holds a list of closures which execute the decoded instruciton
const DECODED_INSTRUCTIONS: [fn(&mut ChipSystem, u16) -> Result<(), String>; 16] = [
    |system, input| { //instruction 0
	match input & 0x0FFF {
	    0x0E0 => {
		system.video.clear_buffer();
		system.video.update_screen();
	    }
	    _ => return Err(format!("unimplemented usage of opcode {:#06x}", input))
	}
	return Ok(());
    },
    |system, input|  { //instruction 1
	system.program_counter = input & 0x0FFF;
	return Ok(());
    },
    |system, input| { //instruciton 2
	return Err("unimplemented usage of opcode 0x2XXX".to_string());
    },
    |system, input| { //instruction 3
	return Err("unimplemented usage of opcode 0x3XXX".to_string());
    },
    |system, input| { //instruction 4
	return Err("unimplemented usage of opcode 0x4XXX".to_string());
    },
    |system, input| { //instruciton 5
	return Err("unimplemented usage of opcode 0x5XXX".to_string());
    },
    |system, input| { //instruction 6
	system.registers.variable_register[((input & 0xF00) >> 8) as usize] = (input & 0xFF) as u8;
	return Ok(());
    },
    |system, input| { //instruction 7
	let register_ref = &mut system.registers.variable_register[((input & 0xF00) >> 8) as usize];
	(*register_ref) = (*register_ref).wrapping_add((input & 0xFF) as u8);
	return Ok(());
    },
    |system, input| { //instruciton 8
	return Err("unimplemented usage of opcode 0x8XXX".to_string());
    },
    |system, input| { //instruciton 9
	return Err("unimplemented usage of opcode 0x9XXX".to_string());
    },
    |system, input| { //instruction A
	system.registers.index_register = input & 0x0FFF;
	return Ok(());
    },
    |system, input| { //instruciton B
	return Err("unimplemented usage of opcode 0xBXXX".to_string());
    },
    |system, input| { //instruciton C
	return Err("unimplemented usage of opcode 0xCXXX".to_string());
    },
    |system, input| { //instruciton D
	let vx = system.registers.variable_register[((input & 0xF00) >> 8) as usize];
	let vy = system.registers.variable_register[((input & 0x0F0) >> 4) as usize];
	let mut height = input & 0xF;
	let i = system.registers.index_register;
	let sprite_lines = &system.ram.memory_array[(i as usize)..((i + height) as usize)] as *const [u8];
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
	return Err("unimplemented usage of opcode 0xEXXX".to_string());
    },
    |system, input| { //instruciton F
	return Err("unimplemented usage of opcode 0xFXXX".to_string());
    }
];

///Contains all the components nessecary to run a chip 8 program
#[derive(Debug)]
pub struct ChipSystem {
    program_counter: u16,
    registers: memory::RegisterSet,
    stack: memory::Stack,
    ram: memory::EntireMemory,
    video: video::VideoDisplay,
    sound_timer: timers::Timer,
    delay_timer: timers::Timer
}

impl ChipSystem {
    pub fn new() -> Self {
	return ChipSystem {
	    program_counter: 0,
	    registers: memory::RegisterSet::new(),
	    stack: memory::Stack::new(),
	    ram: memory::EntireMemory::new(),
	    video: video::VideoDisplay::new(video::TerminalDriver::new()),
	    sound_timer: timers::sound_timer(),
	    delay_timer: timers::delay_timer()
	}
    }

    pub fn load_program(&mut self, program_array: Vec<u8>) {
	self.program_counter = self.ram.load_program(program_array);
    }

    ///decodes the next instruction at the program_counter.
    pub fn decode_next_instruction(&mut self) -> Result<(), String> {
	let instruction_first_byte = self.ram.memory_array[self.program_counter as usize] as u16;
	let instruction_second_byte = self.ram.memory_array[(self.program_counter + 1) as usize] as u16;
	let combined_instruction = (instruction_first_byte << 8) + instruction_second_byte;
	self.program_counter += 2;
	return match DECODED_INSTRUCTIONS[(instruction_first_byte >> 4) as usize](self, combined_instruction) {
	    Ok(_) => Ok(()),
	    Err(error) => Err(format!("{}, instruction was {:#06x}", error, combined_instruction))
	}
    }
}

