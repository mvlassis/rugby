// Simple struct thats holds the current input state of the Gameboy
// Note that true means pressed, false means not pressed
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

// Another simple struct to handle input regarding the emulator state
#[derive(Clone, Copy)]
pub struct EmulatorInput {
	pub exit: bool,
	pub toggle_mute: bool,
	pub toggle_channel: [bool; 4],
    pub prev_bg_map: bool,
    pub next_bg_map: bool,
}

impl EmulatorInput {
    pub fn new() -> Self {
        EmulatorInput {
			exit: false,
			toggle_mute: false,
			toggle_channel: [false; 4],
            prev_bg_map: false,
            next_bg_map: false,
        }
    }
}
