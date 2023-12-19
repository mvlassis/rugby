use std::fs::OpenOptions;
use std::io::Write;
use crate::cartridge::Cartridge;
use crate::input::Input;
use crate::save_state::MMUState;
use crate::timer::Timer;

const MEMORY_SIZE: usize = 65536;

// Gameboy does not actually have an MMU, don't tell the Nintendo ninjas
pub struct MMU {
	pub cartridge: Box<dyn Cartridge>,
	wram:         [u8; 8192],
	io_registers: [u8; 128],
	hram:         [u8; 127],
	ie_register: u8,

	pub timer: Timer,
	input: Input,
	prev_p1: u8,
	pub joypad_interrupt: bool,
	serial_buffer: [u8; 100], // TODO: properly implement
	serial_file_path: String,
}

impl MMU {
	pub fn new(cartridge: Box<dyn Cartridge>) -> Self {
		let serial_file_path = "output.txt";
		MMU {
			cartridge,
			wram: [0; 8192],
			io_registers: [0; 128],
			hram: [0; 127],
			ie_register: 0,
			
			timer: Timer::new(),
			input: Input::new(),
			prev_p1: 0xCF,
			joypad_interrupt: false,
			serial_buffer: [0; 100],
			serial_file_path: serial_file_path.to_string(),
		}
	}

	pub fn initialize(&mut self)  {
		self.io_registers[0x00] = 0xCF; // P1
		self.io_registers[0x02] = 0x7E; // SC
		self.io_registers[0x04] = 0xAB; // DIV
		self.io_registers[0x07] = 0xF8; // TAC
		self.io_registers[0x0F] = 0xE1; // IF

		self.timer.initialize();
	}

	// Get 8-bit value from memory at a specific address
	pub fn get_byte(&self, address: u16) -> u8 {
		if (address as usize) >= MEMORY_SIZE {
			panic!("MMU::get_byte(): Out of memory at address: {:04X}", address);
		}
		match address {
			0x0000..=0x7FFF => self.cartridge.read(address),
			0xA000..=0xBFFF => self.cartridge.read(address),
			0xC000..=0xDFFF => self.wram[address as usize - 0xC000],
			0xE000..=0xFDFF => self.wram[address as usize - 0xE000],
			0xFF00..=0xFF7F => {
				match address {
					0xFF00 =>  {
						self.io_registers[0x00]
					}
					0xFF04 => self.timer.div,
					0xFF05 => self.timer.tima,
					0xFF06 => self.timer.tma,
					0xFF07 => self.timer.tac,
					_ => self.io_registers[address as usize - 0xFF00],
				}
			},
			0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
			0xFFFF => self.ie_register,
			_ => panic!("MMU::get_byte(): Out of memory at address: {:04X}", address),
		}
	}

	// Set an 8-bit value at a specific address in memory
	pub fn set_byte(&mut self, address: u16, value: u8) {
		match address {
			0x0000..=0x7FFF => self.cartridge.write(address, value),
			0xA000..=0xBFFF => self.cartridge.write(address, value),
			0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value,
			0xE000..=0xFDFF => (), // ECHO RAM, ignore
			0xFEA0..=0xFEFF => (), // Prohibited area, ignore
			0xFF00..=0xFF7F => {
				match address {
					0xFF00 => {
						self.io_registers[0x00] = value;
						self.update_p1();
					}
					0xFF01 => self.serial_buffer[0] = value,
					0xFF02 => {
						let mut file = OpenOptions::new()
							.create(true).write(true).append(true)
							.open(&self.serial_file_path).unwrap();
						if let Err(e) = write!(file, "{}",
											   self.serial_buffer[0] as char) {
							eprintln!("Writing error: {}", e.to_string())
						}
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
			_ => unreachable!("MMU::set_byte(): Out of memory at address: {:04X}", address),
		}
	}

	pub fn update_p1(&mut self) {
		let mut input_byte = self.io_registers[0x00];
		let bit4 = MMU::get_bit(self.io_registers[0x00], 4);
		let bit5 = MMU::get_bit(self.io_registers[0x00], 5);
		if bit5 == 1 && bit4 == 1 {
			input_byte = MMU::set_bit(input_byte, 0, 1);
			input_byte = MMU::set_bit(input_byte, 1, 1);
			input_byte = MMU::set_bit(input_byte, 2, 1);
			input_byte = MMU::set_bit(input_byte, 3, 1);
		} else if bit4 == 0 && bit5 == 0 {
			let bit0 = MMU::reverse_flag(self.input.right) & MMU::reverse_flag(self.input.a);
			input_byte = MMU::set_bit(input_byte, 0, bit0);
			let bit1 = MMU::reverse_flag(self.input.left) & MMU::reverse_flag(self.input.b);
			input_byte = MMU::set_bit(input_byte, 1, bit1);
			let bit2 = MMU::reverse_flag(self.input.up) & MMU::reverse_flag(self.input.select);
			input_byte = MMU::set_bit(input_byte, 2, bit2);
			let bit3 = MMU::reverse_flag(self.input.down) & MMU::reverse_flag(self.input.start);
			input_byte = MMU::set_bit(input_byte, 3, bit3);
		} else if bit4 == 0 {
			input_byte = MMU::set_bit(input_byte, 0, MMU::reverse_flag(self.input.right)); 
			input_byte = MMU::set_bit(input_byte, 1, MMU::reverse_flag(self.input.left));
			input_byte = MMU::set_bit(input_byte, 2, MMU::reverse_flag(self.input.up));
			input_byte = MMU::set_bit(input_byte, 3, MMU::reverse_flag(self.input.down)); 
		} else if bit5 == 0 {
			input_byte = MMU::set_bit(input_byte, 0, MMU::reverse_flag(self.input.a));
			input_byte = MMU::set_bit(input_byte, 1, MMU::reverse_flag(self.input.b));
			input_byte = MMU::set_bit(input_byte, 2, MMU::reverse_flag(self.input.select));
			input_byte = MMU::set_bit(input_byte, 3, MMU::reverse_flag(self.input.start));
		}
		self.io_registers[0x00] = input_byte;

		// Check whether a p1 interrupt has occured
		if (self.prev_p1 & 0x0F) & !(input_byte & 0x0F) != 0 {
			self.joypad_interrupt = true;
		}
		
		self.prev_p1 = input_byte;
	}
	
	pub fn store_input(&mut self, input: Input) {
		self.input = input;
		self.update_p1();
	}

	// Return 0 if flag is true, 1 otherwise
	fn reverse_flag(flag: bool) -> u8 {
		if flag {
			0
		} else {
			1
		}
	}
	
	// Get bit at a specific position
	fn get_bit(value: u8, bit_position: u8) -> u8 {
		let bit = (value >> bit_position) & 0x1;
		bit as u8
	}

	// Set bit at a specific position to a specific bit value
	fn set_bit(value: u8, bit_position: u8, bit_value: u8) -> u8 {
		let new_value = match bit_value {
			0 => value & !(1 << bit_position),
			1 => value | 1 << bit_position,
			_ => unreachable!("MMU::set_bit()"),
		};
		new_value
	}

	// Creates an MMU state from the MMU
	pub fn create_state(&self) -> MMUState {
		MMUState {
			wram: self.wram.clone(),
			io_registers: self.io_registers.clone(),
			hram: self.hram.clone(),
			ie_register: self.ie_register,
			timer: self.timer.clone(),
			input: self.input.clone(),
			prev_p1: self.prev_p1,
			joypad_interrupt: self.joypad_interrupt,
			serial_buffer: self.serial_buffer.clone(),
			serial_file_path: self.serial_file_path.clone(),
		}
	}

	// Loads an MMUState to the MMU
	pub fn load_state(&mut self, mmu_state: MMUState) {
		self.wram = mmu_state.wram.clone();
		self.io_registers = mmu_state.io_registers.clone();
		self.hram = mmu_state.hram.clone();
		self.ie_register = mmu_state.ie_register;
		self.timer = mmu_state.timer.clone();
		self.input = mmu_state.input.clone();
		self.prev_p1 = mmu_state.prev_p1;
		self.joypad_interrupt = mmu_state.joypad_interrupt;
		self.serial_buffer = mmu_state.serial_buffer.clone();
		self.serial_file_path = mmu_state.serial_file_path.clone();
	}
}
