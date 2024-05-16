
//rust CHIP8 emulator

use std::time::Duration;
use std::thread;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
};

/// This is the main function for the emulatort
fn main() {
    println!("Starting emulator");

    let sdl_context = sdl2::init().expect("failed to initialize sdl2 library");
    let video_subsystem = sdl_context.video().expect("failed to initialize video subsystem");

    let mut event_pump = sdl_context.event_pump().expect("could not get event pump");
    
    let programwindow = video_subsystem.window("new window thing", 800, 600)
        .position_centered()
        .build()
        .expect("failed to create the window");

    let mut canvas = programwindow.into_canvas()
        .build()
        .expect("failed to build canvas");
    
    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.clear();
    canvas.present();
    
    let mut i: u8 = 0;
    'programloop: loop {
	i = (i + 1) % 255;
	canvas.set_draw_color(Color::RGB(255, i, 255 - i));
	for event in event_pump.poll_iter() {
	    match event {
		Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { break 'programloop},
		_ => continue
	    }
	}
	canvas.present();
	thread::sleep(Duration::from_millis(10));
    }
    
}
