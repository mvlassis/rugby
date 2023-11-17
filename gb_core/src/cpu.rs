mod instructions;

use crate::mmu::MMU;

const EXPANDED_INSTRUCTION_OPCODE: u8 = 0xCB;

pub struct CPU {
	// 7 8-bit registers + 2 for the stack pointer + 1 for the flag register
	cpu_registers: [u8; 10],
	pc: u16,
	mcycles: u8,
	// Interrupt master enable flag
	ime: u8,
	lookup_table: [Option<fn(&mut CPU, &mut MMU)>; 256],
	lookup_table2: [Option<fn(&mut CPU, &mut MMU)>; 256],
}


impl CPU {
	pub fn new() -> Self {
		let lookup_table: [Option<fn(&mut CPU, &mut MMU)>; 256] = [None; 256];
		let lookup_table2: [Option<fn(&mut CPU, &mut MMU)>; 256] = [None; 256];
		CPU {
			cpu_registers: [0; 10],
			pc: 0,
			mcycles: 0,
			ime: 0,
			lookup_table,
			lookup_table2,
		}
	}

	// Initializes registers, PC, and lookup table
	pub fn initialize(&mut self) {
		self.cpu_registers = [0x01, 0xB0, 0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D,
							  0xFF, 0xFE];
		self.pc = 0x100;
		self.build_lookup_tables();
	}

	pub fn print_state(&self, mmu: &MMU) {
		let byte0 = mmu.get_byte(self.pc);
		let byte1 = mmu.get_byte(self.pc+1);
		let byte2 = mmu.get_byte(self.pc+2);
		let byte3 = mmu.get_byte(self.pc+3);
		println!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:02X}{:02X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
				 self.cpu_registers[0],
				 self.cpu_registers[1], self.cpu_registers[2], self.cpu_registers[3],
				 self.cpu_registers[4], self.cpu_registers[5], self.cpu_registers[6],
				 self.cpu_registers[7], self.cpu_registers[8], self.cpu_registers[9],
				 self.pc, byte0, byte1, byte2, byte3);
	}
	
	// Fetches and executes the next instruction 
	pub fn step(&mut self, mmu: &mut MMU) {
		let opcode = mmu.memory[self.pc as usize];
		self.print_state(mmu);
		if opcode == EXPANDED_INSTRUCTION_OPCODE {
			self.pc += 1;
			let opcode = mmu.memory[self.pc as usize];
			if let Some(instruction) = self.lookup_table2[opcode as usize] {
				self.pc += 1;
				instruction(self, mmu);
			} else {
				panic!("Unimplemented expanded opcode: {:02X}", opcode);
			}
		}
		else if let Some(instruction) = self.lookup_table[opcode as usize] {
			self.pc += 1;
			instruction(self, mmu);
		}
		else {
			panic!("Unimplemented opcode: {:02X}", opcode);
		}
		
	}

	// Fetches a byte from the MMU and moves the PC appropriately
	pub fn fetch_byte(&mut self, mmu: &MMU) -> u8 {
		let byte = mmu.get_byte(self.pc);
		self.pc += 1;
		byte
	}

	// Fetches a word from the MMU and moves the PC appropriately
	pub fn fetch_word(&mut self, mmu: &MMU) -> u16 {
		let word = mmu.get_word(self.pc);
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
	fn push_stack(&mut self, mmu: &mut MMU, value: u16) {
		let sp_value = self.double_register_value("SP");
		mmu.memory[(sp_value) as usize] = ((value & 0xFF00) >> 8) as u8;
		mmu.memory[(sp_value-1) as usize] = (value & 0x00FF) as u8;
		self.set_double_register("SP", (sp_value-2) as u16);
	}

	// Pop a 16-bit immediate from the stack, and return its value
	fn pop_stack(&mut self, mmu: &mut MMU) -> u16 {
		let sp_value = self.double_register_value("SP");
		let lsb = mmu.memory[(sp_value+1) as usize];
		let msb = mmu.memory[(sp_value+2) as usize];
		let value = ((msb as u16) << 8) | lsb as u16;
		self.set_double_register("SP", (sp_value+2) as u16);
		return value;
	}


	// Set a bit in a specified register
	fn set_bit(&mut self, r: char, bit_position: u8, value: u8) {
		let r_idx = self.r_index(r);
		if value == 0 {
			self.cpu_registers[r_idx] &= !(1 << bit_position);
		} else {
			self.cpu_registers[r_idx] |= 1 << bit_position;			
		}
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
		self.set_bit('F', bit_position, value);
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
