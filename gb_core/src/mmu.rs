use std::fs::File;
use std::io::{Write, Result};

use crate::timer::Timer;

const MEMORY_SIZE: usize = 65536;
const ROM_START_ADDRESS: usize = 0x00;

// Gameboy does not actually have an MMU, don't tell the Nintendo ninjas
pub struct MMU {
	rom:          [u8; 32768],
	vram:         [u8; 8192],
	external_ram: [u8; 8192],
	wram:         [u8; 8192],
	oam_memory:   [u8; 160],
	io_registers: [u8; 128],
	hram:         [u8; 127],
	ie_register: u8,

	pub timer: Timer,
	serial_buffer: [u8; 100], // TODO: size
	file: File,
}

impl MMU {
	pub fn new() -> Self {
		let file_path = "output.txt";
		let file = File::create(file_path).unwrap();
		MMU {
			rom: [0; 32768],
			vram: [0; 8192],
			external_ram: [0; 8192],
			wram: [0; 8192],
			oam_memory: [0; 160],
			io_registers: [0; 128],
			hram: [0; 127],
			ie_register: 0,
			
			timer: Timer::new(),
			serial_buffer: [0; 100],
			file,
		}
	}

	pub fn initialize(&mut self)  {
		self.io_registers[0x02] = 0x7E;
		self.io_registers[0x04] = 0xAB; // DIV
		self.io_registers[0x07] = 0xF8; // TAC
		self.io_registers[0x0F] = 0xE1; // IF

		self.timer.initialize();
	}
	
	// Load a rom in memory;
	pub fn load(&mut self, data_buffer: &Vec<u8>) {
		let end = ROM_START_ADDRESS + data_buffer.len();
		self.rom[ROM_START_ADDRESS..end].copy_from_slice(data_buffer);
	}

	// Get 8-bit value from memory at a specific address
	pub fn get_byte(&self, address: u16) -> u8 {
		if (address as usize) >= MEMORY_SIZE {
			panic!("get_byte(): Out of memory at address: {}", address);
		}
		match address {
			0x0000..=0x7FFF => self.rom[address as usize],
			0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
			0xA000..=0xBFFF => self.external_ram[address as usize - 0xA000],
			0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
			0xFE00..=0xFE9F => self.oam_memory[address as usize - 0xFE00],
			0xFF00..=0xFF7F => {
				match address {
					0xFF04 => self.timer.div,
					0xFF05 => self.timer.tima,
					0xFF06 => self.timer.tma,
					0xFF07 => self.timer.tac,
					_ => self.io_registers[address as usize - 0xFF00],
				}
			},
			0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
			0xFFFF => self.ie_register,
			_ => panic!("get_byte(): Out of memory at address: {}", address),
		}
	}

	// Get 16-bit value from memory at a specific address
	pub fn get_word(&self, address: u16) -> u16 {
		let byte1 = self.get_byte(address) as u16;
		let byte2 = self.get_byte(address+1) as u16;
		(byte2 << 8) | byte1
	}

	// Set an 8-bit value at a specific address in memory
	pub fn set_byte(&mut self, address: u16, value: u8) {
		match address {
			0x0000..=0x7FFF => self.rom[address as usize] = value,
			0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = value,
			0xA000..=0xBFFF => self.external_ram[address as usize - 0xA000] = value,
			0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value,
			0xFE00..=0xFE9F => self.oam_memory[address as usize - 0xFE00] = value,
			0xFF00..=0xFF7F => {
				match address {
					0xFF01 => self.serial_buffer[0] = value,
					0xFF02 => {
						write!(self.file, "{}", self.serial_buffer[0] as char);
						()
					},
					0xFF04 => self.timer.reset_timer(),
					0xFF05 => self.timer.tima = value,
					0xFF06 => self.timer.tma = value,
					0xFF07 => self.timer.tac = value,
					_ => self.io_registers[address as usize - 0xFF00] = value,
				}
			},
			0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value,
			0xFFFF => self.ie_register = value,
			_ => panic!("get_byte(): Out of memory at address: {}", address),
		}
	}

	// TODO Debug
	pub fn print_if(&self) {
		print!("IF: {} ", self.io_registers[0x0F]);
	}
}
