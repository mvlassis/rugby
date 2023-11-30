use std::process;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::video::Window;

use gb_core::input::Input;
use gb_core::input::EmulatorInput;

pub struct InputDriver {
    event_pump: EventPump,

    pub input: Input,
    pub emulator_input: EmulatorInput,
}

impl InputDriver {
    pub fn new(sdl: &sdl2::Sdl) -> Self {
        let event_pump = sdl.event_pump().unwrap();
        InputDriver {
            event_pump,
            input: Input::new(),
            emulator_input: EmulatorInput::new()
        }
    }

    pub fn handle_input(&mut self, main_window: &Window) {
        let mut input = Input::new();
        let mut emulator_input = EmulatorInput::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					emulator_input.exit = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let window_id = event.get_window_id().unwrap();
                    if window_id == 2 {
                        emulator_input.prev_bg_map = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let window_id = event.get_window_id().unwrap();
                    if window_id == 2 {
                        emulator_input.next_bg_map = true;
                    }
                },
                _ => ()
            }
        }

        let focused = main_window.has_input_focus();
        if focused {
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
        }
        self.input = input;
        self.emulator_input = emulator_input;
    }
    
}
