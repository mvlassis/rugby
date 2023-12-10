mod video_driver;
mod input_driver;

use std::env;
use std::time::Duration;

use sdl2::audio::{AudioQueue, AudioSpecDesired};

use gb_core::emulator::Emulator;

use crate::video_driver::VideoDriver;
use crate::input_driver::InputDriver;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut video_driver = VideoDriver::new(&sdl_context, 3);
    let mut input_driver = InputDriver::new(&sdl_context);

	// Setup the audio
	let audio_subsystem = sdl_context.audio().expect("Failed to initialize audio");
	let desired_spec = AudioSpecDesired {
		freq: Some(44100),
		channels: Some(2),
		samples: Some(1024),
	};
	let audio_queue: AudioQueue<f32> = audio_subsystem.open_queue(None, &desired_spec)
		.expect("Failed to create audio queue");
	audio_queue.resume();
	let callback = Box::new(move |buffer: &[f32]| {
		while audio_queue.size() > 1024 * 4 * 2 {
			std::thread::sleep(Duration::from_millis(1));
		}
		let _ = audio_queue.queue_audio(buffer);
	});
	
    let args: Vec<String> = env::args().collect();
    let first_arg = &args[1];
    let mut gb = Emulator::new(first_arg, callback);
    
    loop {
        video_driver.start_timer();

        input_driver.handle_input(video_driver.canvas.window());
        let screen_buffer = gb.run(input_driver.input, Some(input_driver.emulator_input));
        
        video_driver.draw_window(&screen_buffer);
        
        let tilemap = gb.get_tilemap();
        video_driver.draw_tilemap(&tilemap);
        let bg_map = gb.get_bg_map();
        video_driver.draw_bg_map(&bg_map);

        // video_driver.sleep_for_frame();
        video_driver.print_fps();
    }
}
