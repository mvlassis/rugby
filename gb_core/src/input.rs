use serde::{Serialize, Deserialize};

// Simple struct thats holds the current input state of the Gameboy
// Note that true means pressed, false means not pressed
#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false
        }
    }
}

// Another simple struct to handle input regarding the emulator's settings
#[derive(Clone, Copy)]
pub struct EmulatorInput {
	pub exit: bool,
	pub rewind: bool,
	pub toggle_mute: bool,
	pub toggle_channel: [bool; 4],
	pub save_state: bool,
	pub load_state: bool,
	pub select_save_state: (bool, usize),
	pub select_load_state: (bool, usize),
	pub toggle_layer: [bool; 3], // 0: Background, 1: Window, 2: Objects
    pub prev_bg_map: bool,
    pub next_bg_map: bool,
}

impl EmulatorInput {
    pub fn new() -> Self {
        EmulatorInput {
			exit: false,
			rewind: false,
			toggle_mute: false,
			toggle_channel: [false; 4],
			save_state: false,
			load_state: false,
			select_save_state: (false, 0),
			select_load_state: (false, 0),
			toggle_layer: [false; 3], // 0: Background, 1: Window, 2: Objects
            prev_bg_map: false,
            next_bg_map: false,
        }
    }
}
