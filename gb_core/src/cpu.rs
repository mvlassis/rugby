mod opcodes;

use crate::bus::Bus;

const EXPANDED_INSTRUCTION_OPCODE: u8 = 0xCB;

pub struct CPU {
	// 7 8-bit registers + 2 for the stack pointer + 1 for the flag register
	cpu_registers: [u8; 10],
	pc: u16,
	mcycles: u8,
	// Interrupt master enable flag
	ime: u8,
	lookup_table: [Option<fn(&mut CPU, &mut Bus)>; 256],
	lookup_table2: [Option<fn(&mut CPU, &mut Bus)>; 256],

	ime_scheduled: bool,
	halt_mode: bool,
	rtc_oscillator: u64,
}


impl CPU {
	pub fn new() -> Self {
		let lookup_table: [Option<fn(&mut CPU, &mut Bus)>; 256] = [None; 256];
		let lookup_table2: [Option<fn(&mut CPU, &mut Bus)>; 256] = [None; 256];
		CPU {
			cpu_registers: [0; 10],
			pc: 0,
			mcycles: 0,
			ime: 0,
			lookup_table,
			lookup_table2,
			ime_scheduled: false,
			halt_mode: false,
			rtc_oscillator: 0,
		}
	}

	// Initializes registers, PC, and lookup table
	pub fn initialize(&mut self) {
		self.cpu_registers = [0x01, 0xB0, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D,
							  0xFF, 0xFE];
		self.pc = 0x100;
		self.build_lookup_tables();
	}

	pub fn print_state(&self, bus: &mut Bus) {
		let byte0 = bus.get_byte(self.pc);
		let byte1 = bus.get_byte(self.pc+1);
		let byte2 = bus.get_byte(self.pc+2);
		let byte3 = bus.get_byte(self.pc+3);
		println!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:02X}{:02X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
				 self.cpu_registers[0],
				 self.cpu_registers[1], self.cpu_registers[2], self.cpu_registers[3],
				 self.cpu_registers[4], self.cpu_registers[5], self.cpu_registers[6],
				 self.cpu_registers[7], self.cpu_registers[8], self.cpu_registers[9],
				 self.pc, byte0, byte1, byte2, byte3);
	}
	
	// Fetches and executes the next instruction 
	pub fn step(&mut self, bus: &mut Bus) {

		if self.halt_mode {
			// If in halt mode, don't fetch the next opcode, just tick the bus
			self.tick(bus);
		}
		else {
			let opcode = bus.get_byte(self.pc);
			// self.print_state(bus);
			if opcode == EXPANDED_INSTRUCTION_OPCODE {
				self.pc += 1;
				self.tick(bus);
				let opcode = bus.get_byte(self.pc);
				if let Some(instruction) = self.lookup_table2[opcode as usize] {
					self.pc += 1;
					self.tick(bus);
					instruction(self, bus);
				} else {
					panic!("Unimplemented expanded opcode: {:02X}", opcode);
				}
			}
			else if let Some(instruction) = self.lookup_table[opcode as usize] {
				self.pc += 1;
				self.tick(bus);
				instruction(self, bus);
			}
			else {
				panic!("Unimplemented opcode: {:02X}", opcode);
			}	
		}
		self.handle_interrupts(bus);
	}

	// Increments all parts by one M-Cycle
	pub fn tick(&mut self, bus: &mut Bus) {
		bus.tick();
		self.rtc_oscillator += 1;
		if self.rtc_oscillator % 1048576 == 0 {
			bus.mmu.cartridge.update_clock();
			self.rtc_oscillator = 0
		}
	}

	// Handle interrupts
	pub fn handle_interrupts(&mut self, bus: &mut Bus) {
		let ie = bus.get_byte(0xFFFF);
		let mut if_register = bus.get_byte(0xFF0F);
		if self.ime == 1 {
			if ie & if_register != 0 {
				if self.halt_mode {
					self.halt_mode = false;
				}
				let interrupt_type = (ie & if_register).trailing_zeros() as u8;
				self.ime = 0; // Disable IME flag
				if_register = self.set_bit(if_register, interrupt_type, 0);
				bus.set_byte(0xFF0F, if_register);

				self.tick(bus);
				self.tick(bus);
				self.push_stack(bus, self.pc);
				self.pc = match interrupt_type {
					0 => 0x0040, // VBlank
					1 => 0x0048, // STAT
					2 => 0x0050, // Timer
					3 => 0x0058, // Serial
					4 => 0x0060, // Joypad
					_ => panic!("No interrupt type found: {}", interrupt_type),
				};
				self.tick(bus);
			}
		}
		// The CPU will exit halt mode even if IME is set to 0
		// TODO: Halt bug
		else if ie & if_register != 0 {
			if self.halt_mode {
				self.halt_mode = false;
			}
		}
		if self.ime_scheduled {
			self.ime = 1;
			self.ime_scheduled = false;
		}
	}

	// Fetches a byte from the MMU and moves the PC appropriately
	pub fn fetch_byte(&mut self, bus: &mut Bus) -> u8 {
		let byte = bus.get_byte(self.pc);
		self.tick(bus);
		self.pc += 1;
		byte
	}

	// Gets a byte from the CPU
	pub fn get_byte(&mut self, bus: &mut Bus, mem: u16) -> u8 {
		let byte = bus.get_byte(mem);
		self.tick(bus);
		byte
	}

	// Sets a byte in the CPU
	pub fn set_byte(&mut self, bus: &mut Bus, mem: u16, value: u8) {
		bus.set_byte(mem, value);
		self.tick(bus);
	}

	// Fetches a word from the MMU and moves the PC appropriately
	pub fn fetch_word(&mut self, bus: &mut Bus) -> u16 {
		let word = bus.get_word(self.pc);
		self.tick(bus);
		self.tick(bus);
		self.pc += 2;
		word
	}
	
	// Returns the proper index of the register character
	fn r_index(&self, reg_name: char) -> usize {
		match reg_name {
			'A' => 0,
			'F' => 1,
			'B' => 2,
			'C' => 3,
			'D' => 4,
			'E' => 5,
			'H' => 6,
			'L' => 7,
			'S' => 8,
			'P' => 9,
			_ => panic!("Invalid register name: {}", reg_name),
		}
	}

	// Returns the value of the double register
	fn double_register_value(&self, double_register_name: &str) -> u16 {
		if double_register_name.len() != 2 {
			panic!("Invalid double register name: {}", double_register_name)
		}

		let high_register_char = double_register_name.chars().nth(0).unwrap();
		let low_register_char = double_register_name.chars().nth(1).unwrap();

		let high_index = self.r_index(high_register_char);
		let low_index = self.r_index(low_register_char);

		let value = ((self.cpu_registers[high_index] as u16) << 8) | self.cpu_registers[low_index] as u16;
		value
	}

	// Set the value of the double register
	fn set_double_register(&mut self, double_register_name: &str, value: u16) {
		if double_register_name.len() != 2 {
			panic!("Invalid double register name: {}", double_register_name)
		}
		
        let high_register_char = double_register_name.chars().nth(0).unwrap();
		let low_register_char = double_register_name.chars().nth(1).unwrap();

		let high_index = self.r_index(high_register_char);
		let low_index = self.r_index(low_register_char);

        // Set the value of the double register
        self.cpu_registers[high_index] = ((value & 0xFF00) >> 8) as u8;
        self.cpu_registers[low_index] = (value &0x00FF) as u8;
    }

	// Push a 16-bit immediate to the stack
	fn push_stack(&mut self, bus: &mut Bus, value: u16) {
		let sp_value = self.double_register_value("SP");
		bus.set_byte(sp_value.wrapping_sub(1), ((value & 0xFF00) >> 8) as u8);
		self.tick(bus);
		bus.set_byte(sp_value.wrapping_sub(2), (value & 0x00FF) as u8);
		self.tick(bus);
		self.set_double_register("SP", (sp_value.wrapping_sub(2)) as u16);
	}

	// Pop a 16-bit immediate from the stack, and return its value
	fn pop_stack(&mut self, bus: &mut Bus) -> u16 {
		let sp_value = self.double_register_value("SP");
		let lsb = bus.get_byte(sp_value);
		self.tick(bus);
		let msb = bus.get_byte(sp_value+1);
		self.tick(bus);
		let value = ((msb as u16) << 8) | lsb as u16;
		self.set_double_register("SP", (sp_value+2) as u16);
		return value;
	}

	// Set a bit in a u8
	fn set_bit(&mut self, value: u8, bit_position: u8, bit_value: u8) -> u8 {
		let new_value = match bit_value {
			0 => value & !(1 << bit_position),
			1 => value | 1 << bit_position,
			_ => panic!("Set bit"),
		};
		new_value
	}
	
	// Set a flag to the specified value
	fn set_flag(&mut self, flag_name: char, value: u8) {
		let bit_position = match flag_name {
			'z' => 7,
			'n' => 6,
			'h' => 5,
			'c' => 4,
			_ => panic!("Invalid flag name: {}", flag_name),
		};
		let f_idx = self.r_index('F');
		self.cpu_registers[f_idx] = self.set_bit(self.cpu_registers[f_idx], bit_position, value);
	}

	// Return the value of a specific flag
	fn get_flag(&mut self, flag_name: char) -> u8 {
		let bit_position = match flag_name {
			'z' => 7,
			'n' => 6,
			'h' => 5,
			'c' => 4,
			_ => panic!("Invalid flag name: {}", flag_name),
		};
		let flag_reg = self.r_index('F');
		let bit = (self.cpu_registers[flag_reg] & (1 << bit_position)) >> bit_position;
		bit
	}

}
