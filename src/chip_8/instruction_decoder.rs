
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
	let vy = system.registers.variable_register[((input & 0x0F0) >> 4) as usize];
	let height = input & 0xF;
	let i = system.registers.index_register;
	let sprite_lines = &system.ram.memory_array[(i as usize)..((i + height - 1) as usize)] as *const [u8];
	system.video.draw_sprite(vx, vy, Box::new(move || {
	    if height > 0 {
		unsafe {
		    return Some(((*sprite_lines)[(height - 1) as usize], (height - 1) as u8));
		}
	    }
	    return None;
	}));
	system.program_counter += 1;
	return Ok(());
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
    pub fn decodeInstruction(system: &mut ChipSystem, input: u16) -> Result<(), String> {
	return DECODED_INSTRUCTIONS[((input & 0xF000) >> 12) as usize](system, input);
    }
}

