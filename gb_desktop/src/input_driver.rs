use std::process;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputDriver {
	event_pump: EventPump,
}

impl InputDriver {
	pub fn new(sdl: &sdl2::Sdl) -> Self {
		let event_pump = sdl.event_pump().unwrap();
		InputDriver {
			event_pump
		}
	}

	pub fn handle_input(&mut self) {
		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					process::exit(0);
				},
				_ => ()
			}
		}
	}
}
