
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
	    _ => return Err("unimplemented usage of opcode 0x0XXX".to_string())
	}
	system.program_counter += 1;
	return Ok(());
    },
    |system, input|  { //instruction 1
	system.program_counter = input & 0x0FFF;
	return Ok(());
    },
    |system, input| { //instruciton 2
	unimplemented!();
    },
    |system, input| { //instruction 3
	unimplemented!();
    },
    |system, input| { //instruction 4
	unimplemented!();
    },
    |system, input| { //instruciton 5
	unimplemented!();
    },
    |system, input| { //instruction 6
	system.registers.variable_register[((input & 0xF00) >> 8) as usize] = (input & 0xFF) as u8;
	system.program_counter += 1;
	return Ok(());
    },
    |system, input| { //instruction 7
	let register_ref = &mut system.registers.variable_register[((input & 0xF00) >> 8) as usize];
	(*register_ref) = (*register_ref).wrapping_add((input & 0xFF) as u8);
	system.program_counter += 1;
	return Ok(());
    },
    |system, input| { //instruciton 8
	unimplemented!();
    },
    |system, input| { //instruciton 9
	unimplemented!();
    },
    |system, input| { //instruction A
	system.registers.index_register = input & 0x0FFF;
	system.program_counter += 1;
	return Ok(());
    },
    |system, input| { //instruciton B
	unimplemented!();
    },
    |system, input| { //instruciton C
	unimplemented!();
    },
    |system, input| { //instruciton D
	let vx = system.registers.variable_register[((input & 0xF00) >> 8) as usize];
	let vy = system.registers.variable_register[((input & 0x0F0) >> 8) as usize];
	
	unimplemented!();
    },
    |system, input| { //instruciton E
	unimplemented!();
    },
    |system, input| { //instruciton F
	unimplemented!();
    }
];

pub struct ChipSystem {
    program_counter: u16,
    registers: memory::RegisterSet,
    stack: memory::Stack,
    ram: memory::EntireMemory,
    video: video::VideoDisplay,
    sound_timer: timers::Timer,
    delay_timer: timers::Timer
}

struct Instructions;

impl Instructions {
    pub fn decodeInstruction(system: &mut ChipSystem, input: u16) -> Result<(), String> {
	return DECODED_INSTRUCTIONS[((input & 0xF000) >> 12) as usize](system, input);
    }
}

