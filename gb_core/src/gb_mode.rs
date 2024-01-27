use serde::{Serialize, Deserialize};

// Represents the current emulator mode
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum GBMode {
	DMG, // For original Gameboy systems
	CGB, // For Gameboy Color systems
}
