use std::fs::File;
use std::io::{Write, Result};

const MEMORY_SIZE: usize = 65536;
const ROM_START_ADDRESS: usize = 0x00;

// Gameboy does not actually have an MMU, don't tell the Nintendo ninjas
pub struct MMU {
	pub memory: [u8; MEMORY_SIZE],
	serial_buffer: [u8; 100], // TODO: size
	file: File,
}

impl MMU {
	pub fn new() -> Self {
		let file_path = "output.txt";
		let file = File::create(file_path).unwrap();
		MMU {
			memory: [0; MEMORY_SIZE],
			serial_buffer: [0; 100],
			file,
		}
	}

	// Load a rom in memory;
	pub fn load(&mut self, data_buffer: &Vec<u8>) {
		let end = ROM_START_ADDRESS + data_buffer.len();
		self.memory[ROM_START_ADDRESS..end].copy_from_slice(data_buffer);
	}

	// Get 8-bit value from memory at a specific address
	pub fn get_byte(&self, address: u16) -> u8 {
		if (address as usize) >= MEMORY_SIZE {
			panic!("get_byte(): Out of memory at address: {}", address);
		}
		// if address == 0xFF80 {
		// 	return 0x90;
		// }
		self.memory[address as usize]
	}

	// Set an 8-bit value at a specific address in memory
	pub fn set_byte(&mut self, address: u16, value: u8) {
		if address as usize >= MEMORY_SIZE {
			panic!("set_byte(): Out of memory at address: {}", address);
		}
		// If writing to serial transfer data (SB)
		if address == 0xFF01 {
			self.serial_buffer[0] = value;
		}
		// If wirting to serial transfer control (SC)
		if address == 0xFF02 {
			write!(self.file, "{}", self.serial_buffer[0] as char);
		}
		self.memory[address as usize] = value;
	}

	// Get 16-bit value from memory at a specific address
	pub fn get_word(&self, address: u16) -> u16 {
		if (address as usize) >= MEMORY_SIZE {
			panic!("get_word(): Out of memory at address: {}", address);
		}
		let byte1 = self.memory[address as usize] as u16;
		let byte2 = self.memory[(address+1) as usize] as u16;
		(byte2 << 8) | byte1 // Little endian
	}

	// Set 16-bit value at a specific address in memory
	pub fn set_word(&mut self, address: u16, value: u16) {
		if (address as usize) >= MEMORY_SIZE {
			panic!("set_word(): Out of memory at address: {}", address);
		}
		let byte1 = (value & 0xFF) as u8;
		let byte2 = ((value >> 8) & 0xFF) as u8;
		self.memory[address as usize] = byte1;
		self.memory[address as usize] = byte2;
	}
}
