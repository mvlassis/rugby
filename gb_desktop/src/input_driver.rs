use std::process;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::video::Window;

use gb_core::input::Input;

pub struct InputDriver {
    event_pump: EventPump,
    // main_window: &Window,
}

impl InputDriver {
    pub fn new(sdl: &sdl2::Sdl) -> Self {
        let event_pump = sdl.event_pump().unwrap();
        InputDriver {
            event_pump,
        }
    }

    pub fn handle_input(&mut self) -> Input {
        let mut input = Input::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(0);
                },
                _ => ()
            }
        }
        let keyboard_state = self.event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(Scancode::Up) {
            input.up = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Down) {
            input.down = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            input.left = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            input.right = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            input.a = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            input.b = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Z) {
            input.start = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::X) {
            input.select = true;
        }
        input
    }
}
