mod video_driver;
mod input_driver;

use std::env;

use gb_core::emulator::Emulator;

use crate::video_driver::VideoDriver;
use crate::input_driver::InputDriver;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut video_driver = VideoDriver::new(&sdl_context, 3);
    let mut input_driver = InputDriver::new(&sdl_context);
    let args: Vec<String> = env::args().collect();
    let first_arg = &args[1];
    let mut gb = Emulator::new(first_arg);
    
    loop {
        video_driver.start_timer();

        input_driver.handle_input(video_driver.canvas.window());
        let screen_buffer = gb.run(input_driver.input, Some(input_driver.emulator_input));
        
        video_driver.draw_window(&screen_buffer);
        
        let tilemap = gb.get_tilemap();
        video_driver.draw_tilemap(&tilemap);
        let bg_map = gb.get_bg_map();
        video_driver.draw_bg_map(&bg_map);

        video_driver.sleep_for_frame();
        // video_driver.print_fps();
    }
}
