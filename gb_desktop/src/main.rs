mod video_driver;
mod input_driver;

use std::env;

use gb_core::emulator::Emulator;

use crate::video_driver::VideoDriver;
use crate::input_driver::InputDriver;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let mut video_driver = VideoDriver::new(&sdl_context, 2);
	let mut input_driver = InputDriver::new(&sdl_context);
	let args: Vec<String> = env::args().collect();
	let first_arg = &args[1];
	let mut gb = Emulator::new();
	gb.load(first_arg);
	
	loop {
		video_driver.start_timer();
		input_driver.handle_input();
		let screen_buffer = gb.run();
		video_driver.draw_window(&screen_buffer);
		let tilemap = gb.get_tilemap();
		video_driver.draw_tilemap(&tilemap);
		// video_driver.print_fps();
	}
}
