use crate::mmu::MMU;

pub struct CPU {
	// 7 8-bit registers + 2 for the stack pointer + 1 for the flag register
	cpu_registers: [u8; 10],
	pc: u16,
	// Interrupt master enable flag
	ime: u8,
	lookup_table: [Option<OpcodeHandler>; 256],	
}


#[derive(Clone, Copy)]
enum OpcodeHandler {
	NoArg(fn(&mut CPU)),
	OneArg(fn(&mut CPU, char)),
	TwoArgs(fn(&mut CPU, char, char)),
}

impl CPU {
	pub fn new() -> Self {
		let mut lookup_table: [Option<OpcodeHandler>; 256] = [None; 256];
		CPU {
			cpu_registers: [0; 10],
			pc: 0,
			ime: 0,
			lookup_table,
		}
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
	fn double_register_value(&self, double_register_name: &str) -> usize {
		if double_register_name.len() != 2 {
			panic!("Invalid double register name: {}", double_register_name)
		}

		let high_register_char = double_register_name.chars().nth(0).unwrap();
		let low_register_char = double_register_name.chars().nth(1).unwrap();

		let high_index = self.r_index(high_register_char);
		let low_index = self.r_index(low_register_char);

		let value = ((self.cpu_registers[high_index] as u16) << 8) | self.cpu_registers[low_index] as u16;
		value as usize
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
	fn push_stack(&mut self, value: u16) {
		let sp_value = self.double_register_value("SP");
		self.memory[sp_value-1] = ((value & 0xFF00) >> 8) as u8;
		self.memory[sp_value-2] = (value & 0x00FF) as u8;
		self.set_double_register("SP", (sp_value-2) as u16);
	}

	// Pop a 16-bit immediate from the stack, and return its value
	fn pop_stack(&mut self) -> u16 {
		let sp_value = self.double_register_value("SP");
		let lsb = self.memory[sp_value];
		let msb = self.memory[sp_value+1];
		let value = ((msb as u16) << 8) | lsb as u16;
		return value;
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
		let flag_reg = self.r_index('F');
		if value == 0 {
			self.cpu_registers[flag_reg] &= !(1 << bit_position);
		} else {
			self.cpu_registers[flag_reg] |= 1 << bit_position;			
		}
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

	
	// LD r, r': Load register (register)
	fn opcode_ld_rr(&mut self, dest: char, src: char) {
		let dest_register = self.r_index(dest);
		let src_register = self.r_index(src);		
		self.cpu_registers[dest_register] = self.cpu_registers[src_register];
	}

	// LD r, nn: Load register (immediate)
	fn opcode_ld_rn(&mut self, r: char, n: u8) {
		let dest_register = self.r_index(r);
		self.cpu_registers[dest_register] = n;
	}

	// LD r, m: Load register from memory pointed to by double register
	fn opcode_ld_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.cpu_registers[r_reg] = self.memory[mem];
	}

	// LD m, r: Load memory pointed by double register from register
	fn opcode_ld_mr(&mut self, double_reg: &str, r: char) {
		let src_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.memory[mem] = self.cpu_registers[src_register];
	}

	// LD m, n: Load memory pointed by double register from immediate
	fn opcode_ld_mn(&mut self, double_reg: &str, n: u8) {
		let mem = self.double_register_value(double_reg);
		self.memory[mem] = n;
	}

	// Load r, nn: Load register from memory pointed by 16-bit immediate
	fn opcode_ld_rnn(&mut self, r: char, nn: u16) {
		let dest_register = self.r_index(r);
		self.cpu_registers[dest_register] = self.memory[nn as usize];
	}

	// Load nn, r: Load memory pointed by 16-bit immediate from register
	fn opcode_ld_nnr(&mut self, nn: u16, r: char) {
		let src_register = self.r_index(r);
		self.memory[nn as usize] = self.cpu_registers[src_register];
	}

	// LoadHigh r, n: Load register from memory pointed by 0xFF00 + n
	fn opcode_ldh_rn(&mut self, r: char, n: u8) {
		let dest_register = self.r_index(r);
		let mem = 0xFF00 | (n as u16);
		self.cpu_registers[dest_register] = self.memory[mem as usize];
	}

	// LoadHigh n, r: Load memory pointed by 0xFF00 + n from register
	fn opcode_ldh_nr(&mut self, n: u8, r: char) {
		let mem = 0xFF00 | (n as u16);
		let src_register = self.r_index(r);
		self.memory[mem as usize] = self.cpu_registers[src_register];
	}

	// LoadHigh r, m: Load register from memory poitned by 0xFF00 + register
	fn opcode_ldh_rm(&mut self, r: char, src: char) {
		let dest_register = self.r_index(r);
		let src_register = self.r_index(src);
		let mem = 0xFF00 | (self.cpu_registers[src_register] as u16);
		self.cpu_registers[dest_register] = self.memory[mem as usize];
	}

	// LoadHigh m, r: Load memory pointed by 0xFF00 + register from register
	fn opcode_ldh_mr(&mut self, dest: char, r: char) {
		let dest_register = self.r_index(dest);
		let src_register = self.r_index(r);
		let mem = 0xFF00 | (self.cpu_registers[src_register] as u16);
		self.cpu_registers[dest_register] = self.memory[mem as usize];
	}

	// LoadIncrement m, r: Load memory pointed by double register from register,
	// then increment double register
	fn opcode_ldi_mr(&mut self, double_reg: &str, r: char) {
		let src_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.memory[mem] = self.cpu_registers[src_register];
		self.set_double_register(double_reg, (mem+1) as u16);
	}

	// LoadIncrement r, m: Load register from memory pointed by double register,
	// then increment double register
	fn opcode_ldi_rm(&mut self, r: char, double_reg: &str) {
		let dest_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.cpu_registers[dest_register] = self.memory[mem];
		self.set_double_register(double_reg, (mem+1) as u16);
	}

	// LoadDecrement m, r: Load memory pointed by double register from register,
	// then decrement double register
	fn opcode_ldd_mr(&mut self, double_reg: &str, r: char) {
		let src_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.memory[mem] = self.cpu_registers[src_register];
		self.set_double_register(double_reg, (mem-1) as u16);
	}

	// LoadDecrement r, m: Load register from memory pointed by double register,
	// then decrement double register
	fn opcode_ldd_rm(&mut self, r: char, double_reg: &str) {
		let dest_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.cpu_registers[dest_register] = self.memory[mem];
		self.set_double_register(double_reg, (mem-1) as u16);
	}

	// Load rr, nn: Load double register from 16-bit immediate
	fn opcode_ld_rrnn(&mut self, double_reg: &str, nn: u16) {
		self.set_double_register(double_reg, nn);
	}

	// Load nn, rr: Load double to memory from double register
	fn opcode_ld_nnrr(&mut self, nn: u16, double_reg: &str) {
		let value = self.double_register_value(double_reg);
		self.memory[nn as usize] = (value & 0x00FF) as u8;
		self.memory[nn as usize + 1] = ((value & 0xFF00) >> 8) as u8;
	}

	// Load rr, rr': Load double register from double register
	fn opcode_ld_rrrr(&mut self, double_reg1: &str, double_reg2: &str) {
		let value = self.double_register_value(double_reg2);
		self.set_double_register(double_reg1, value as u16);
	}

	// Push rr: Push the value of a double register to the stack
	fn opcode_push_rr(&mut self, double_reg: &str) {
		let value = self.double_register_value(double_reg);
		self.push_stack(value as u16);
	}

	// Pop rr: Pop the stack and store its value in a double register
	fn opcode_pop_rr(&mut self, double_reg: &str) {
		let value = self.pop_stack();
		self.set_double_register(double_reg, value);
	}

	// Add r, r: Add register to register
	fn opcode_add_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let (result, overflow) = a.overflowing_add(b);
		self.cpu_registers[r1_reg] = result;

		let hc = (((a & 0xF) + (b & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// Add r, n: Add immediate to register
	fn opcode_add_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_add(n);
		self.cpu_registers[r_reg] = result;

		let hc = (((self.cpu_registers[r_reg] & 0xf) + (n & 0xf)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// Add r, m : Add from memory pointed by dobule register to register
	fn opcode_add_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_add(self.memory[mem]);
		self.cpu_registers[r_reg] = result;
		let hc = (((self.cpu_registers[r_reg] & 0xF) + (self.memory[mem] & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// Adc r, r: Add with carry from register to register
	fn opcode_adc_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_add(b);
		let (result2, overflow2) = result1.overflowing_add(carry);
		self.cpu_registers[r1_reg] = result2;
		let hc = (((a & 0xF) + (b & 0xF) + (carry & 0xF)) & 0x10) >> 4;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// Adc r, n: Add with carry from immediate to register
	fn opcode_adc_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let a = self.cpu_registers[r_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_add(n);
		let (result2, overflow2) = result1.overflowing_add(carry);
		self.cpu_registers[r_reg] = result2;
		let hc = (((a & 0xF) + (n & 0xF) + (carry & 0xF)) & 0x10) >> 4;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// Adc r, m: Add with carry from memory to register
	fn opcode_adc_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let a = self.cpu_registers[r_reg];
		let b = self.memory[mem];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_add(b);
		let (result2, overflow2) = result1.overflowing_add(carry);
		self.cpu_registers[r_reg] = result2;
		let hc = (((a & 0xF) + (b & 0xF) + (carry & 0xF)) & 0x10) >> 4;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SUB r, r: Subtract register from register
	fn opcode_sub_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let (result, overflow) = a.overflowing_sub(b);
		self.cpu_registers[r1_reg] = result;

		let hc = (((a & 0xF) - (b & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SUB r, n: Subtract immediate from register
	fn opcode_sub_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_sub(n);
		self.cpu_registers[r_reg] = result;

		let hc = (((self.cpu_registers[r_reg] & 0xf) - (n & 0xf)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SUB r, m : Subtract value pointed by dobule register from register
	fn opcode_sub_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_sub(self.memory[mem]);
		self.cpu_registers[r_reg] = result;
		let hc = (((self.cpu_registers[r_reg] & 0xF) - (self.memory[mem] & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SBC r, r: Subtract with carry, register from register
	fn opcode_sbc_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_sub(b);
		let (result2, overflow2) = result1.overflowing_sub(carry);
		self.cpu_registers[r1_reg] = result2;
		let hc = (((a & 0xF) - (b & 0xF) - (carry & 0xF)) & 0x10) >> 4;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SBC r, n: Subtract with carry immediate from register
	fn opcode_sbc_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let a = self.cpu_registers[r_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_sub(n);
		let (result2, overflow2) = result1.overflowing_sub(carry);
		self.cpu_registers[r_reg] = result2;
		let hc = (((a & 0xF) - (n & 0xF) - (carry & 0xF)) & 0x10) >> 4;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SBC r, m: Subtract with carry, memory from register
	fn opcode_sbc_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let a = self.cpu_registers[r_reg];
		let b = self.memory[mem];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_sub(b);
		let (result2, overflow2) = result1.overflowing_sub(carry);
		self.cpu_registers[r_reg] = result2;
		let hc = (((a & 0xF) - (b & 0xF) - (carry & 0xF)) & 0x10) >> 4;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// AND r, r: AND register with register
	fn opcode_and_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let result = self.cpu_registers[r1_reg] & self.cpu_registers[r2_reg];
		self.cpu_registers[r1_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.set_flag('c', 0);
	}

	// AND r, n: AND register with immediate
	fn opcode_and_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let result = self.cpu_registers[r_reg] & n;
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.set_flag('c', 0);
	}

	// AND r, m: AND register with memory pointed by double register
	fn opcode_and_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let result = self.cpu_registers[r_reg] & self.memory[mem];
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.set_flag('c', 0);
	}

	// XOR r, r: XOR register with register
	fn opcode_xor_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let result = self.cpu_registers[r1_reg] ^ self.cpu_registers[r2_reg];
		self.cpu_registers[r1_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// XOR r, n: XOR register with immediate
	fn opcode_xor_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let result = self.cpu_registers[r_reg] ^ n;
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// XOR r, m: XOR register with memory pointed by double register
	fn opcode_xor_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let result = self.cpu_registers[r_reg] ^ self.memory[mem];
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// OR r, r: OR register with register
	fn opcode_or_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let result = self.cpu_registers[r1_reg] | self.cpu_registers[r2_reg];
		self.cpu_registers[r1_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// OR r, n: OR register with immediate
	fn opcode_or_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let result = self.cpu_registers[r_reg] | n;
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// OR r, m: OR register with memory pointed by double register
	fn opcode_or_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let result = self.cpu_registers[r_reg] | self.memory[mem];
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {    
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// CP r, r: Compare register with register
	fn opcode_cp_rr(&mut self, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let (result, overflow) = a.overflowing_sub(b);

		let hc = (((a & 0xF) - (b & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// CP r, n: Compare register with immediate
	fn opcode_cp_rn(&mut self, r: char, n: u8) {
		let r_reg = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_sub(n);

		let hc = (((self.cpu_registers[r_reg] & 0xf) - (n & 0xf)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// CP r, m : Compare register with value pointed by double register
	fn opcode_cp_rm(&mut self, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_sub(self.memory[mem]);
		let hc = (((self.cpu_registers[r_reg] & 0xF) - (self.memory[mem] & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// INC r: Increment register
	fn opcode_inc_r(&mut self, r: char) {
		let r_reg = self.r_index(r);
		let (result, _) = self.cpu_registers[r_reg].overflowing_add(1);
		self.cpu_registers[r_reg] = result;

		let hc = (((self.cpu_registers[r_reg] & 0xF) + (1 & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
	}

	// INC m: Increment memory pointed by double register
	fn opcode_inc_rm(&mut self, double_reg: &str) {
		let mem = self.double_register_value(double_reg);
		let (result, _) = self.memory[mem].overflowing_add(1);
		self.memory[mem] = result;
		
		let hc = (((self.memory[mem] & 0xF) + (1 & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
	}

	// DEC r: Decrement register
	fn opcode_dec_r(&mut self, r: char) {
		let r_reg = self.r_index(r);
		let (result, _) = self.cpu_registers[r_reg].overflowing_sub(1);
		self.cpu_registers[r_reg] = result;

		let hc = (((self.cpu_registers[r_reg] & 0xF) - (1 & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
	}

	// DEC m: Decrement memory pointed by double register
	fn opcode_dec_m(&mut self, double_reg: &str) {
		let mem = self.double_register_value(double_reg);
		let (result, _) = self.memory[mem].overflowing_sub(1);
		self.memory[mem] = result;
		
		let hc = (((self.memory[mem] & 0xF) - (1 & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
	}

	// DAA r: Decimal adjust register
	fn opcode_daa_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let value = self.cpu_registers[r_idx];
		let mut low_nibble = value & 0xF;
		if low_nibble > 9 {
			low_nibble = (low_nibble + 6) % 16;
		}
		let mut high_nibble = value & 0xF0;
		let mut overflow = false;
		if (high_nibble >> 4) > 9 {
			(high_nibble, overflow) = high_nibble.overflowing_add(6 * 16);
		}

		let adjusted_value = high_nibble | low_nibble;
		self.cpu_registers[r_idx] = adjusted_value;

		if adjusted_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('h', 0);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// CPL r: Complement register
	fn opcode_cpl_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let flipped_value = !self.cpu_registers[r_idx];
		self.cpu_registers[r_idx] = flipped_value;

		self.set_flag('n', 1);
		self.set_flag('h', 1);
	}

	// ADD rr, rr: Add double register to double register
	fn opcode_add_rrrr(&mut self, dreg_str1: &str, dreg_str2: &str) {
		let dreg_val1 = self.double_register_value(dreg_str1);
		let dreg_val2 = self.double_register_value(dreg_str2);
		let (result, overflow) = dreg_val1.overflowing_add(dreg_val2);
		self.set_double_register(dreg_str1, result as u16);

		let hc = (((dreg_val1 & 0xFFF) + (dreg_val2 & 0xFFF)) & 0x1000) >> 12;
		
		self.set_flag('n', 0);
		self.set_flag('h', hc as u8);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// INC rr: Increment value of double register
	fn opcode_inc_rr(&mut self, dreg_str: &str) {
		let dreg_val = self.double_register_value(dreg_str);
		let (result, _) = dreg_val.overflowing_add(1);
		self.set_double_register(dreg_str, result as u16);
	}

	// DEC rr: Decrement value of double register
	fn opcode_dec_rr(&mut self, dreg_str: &str) {
		let dreg_val = self.double_register_value(dreg_str);
		let (result, _) = dreg_val.overflowing_sub(1);
		self.set_double_register(dreg_str, result as u16);
	}

	// ADDS m, dd: Add signed 8-bit to double register
	fn opcode_adds(&mut self, dreg_str: &str, dd: u8) {
		let dreg_val = self.double_register_value(dreg_str);
		let signed_value = dd as i8;
		let (result, _) = dreg_val.overflowing_add_signed(signed_value as isize);
		self.set_double_register(dreg_str, result as u16);

		let hc = if signed_value > 0 {
			(((dreg_val & 0xF) + (dd as usize & 0xF)) & 0x10) >> 4
		} else {
			(((dreg_val & 0xF) - (signed_value.abs() as usize & 0xF)) & 0x10) >> 4
		};

		let fc = if signed_value > 0 {
			(((dreg_val & 0xFF) + (dd as usize & 0xFF)) & 0x100) >> 8
		} else {
			(((dreg_val & 0xFF) - (signed_value.abs() as usize & 0xFF)) & 0x100) >> 8
		};
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', hc as u8);
		self.set_flag('c', fc as u8);
	}
	
	// LDS m, m, dd: Add signed 8-bit to double register, then store the result in memory
	// pointed by double-register
	fn opcode_lds(&mut self, dreg_str1: &str, dreg_str2: &str, dd: u8) {
		let dreg_val1 = self.double_register_value(dreg_str1);
		let signed_value = dd as i8;
		let (result, _) = dreg_val1.overflowing_add_signed(signed_value as isize);
		self.set_double_register(dreg_str2, result as u16);

		let hc = if signed_value > 0 {
			(((dreg_val1 & 0xF) + (dd as usize & 0xF)) & 0x10) >> 4
		} else {
			(((dreg_val1 & 0xF) - (signed_value.abs() as usize & 0xF)) & 0x10) >> 4
		};

		let fc = if signed_value > 0 {
			(((dreg_val1 & 0xFF) + (dd as usize & 0xFF)) & 0x100) >> 8
		} else {
			(((dreg_val1 & 0xFF) - (signed_value.abs() as usize & 0xFF)) & 0x100) >> 8
		};
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', hc as u8);
		self.set_flag('c', fc as u8);
	}

	// RLCA: Rotate A left
	fn opcode_rlca(&mut self) {
		let r_idx = self.r_index('A');
		let c = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let new_value = self.cpu_registers[r_idx].rotate_left(1);
		self.cpu_registers[r_idx] = new_value;
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RLCA: Rotate A left through carry
	fn opcode_rla(&mut self) {
		let r_idx = self.r_index('A');
		let c = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let mut new_value = self.cpu_registers[r_idx].rotate_left(1);
		new_value |= c;
		self.cpu_registers[r_idx] = new_value;
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RRCA: Rotate A right 
	fn opcode_rrca(&mut self) {
		let r_idx = self.r_index('A');
		let c = self.cpu_registers[r_idx] & 0x01;
		let new_value = self.cpu_registers[r_idx].rotate_right(1);
		self.cpu_registers[r_idx] = new_value;

		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RRA: Rotate A right through carry
	fn opcode_rra(&mut self) {
		let r_idx = self.r_index('A');
		let c = self.cpu_registers[r_idx] & 0x01;
		let mut new_value = self.cpu_registers[r_idx].rotate_right(1);
		new_value |= c << 7;
		self.cpu_registers[r_idx] = new_value;

		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RLC r: Rotate r left
	fn opcode_rlc_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let c = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let new_value = self.cpu_registers[r_idx].rotate_left(1);
		self.cpu_registers[r_idx] = new_value;
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RLC m: Rotate m left
	fn opcode_rlc_m(&mut self, dreg_str: &str) {
		let mem = self.double_register_value(dreg_str);
		let c = (self.memory[mem] & 0x80) >> 7;
		let new_value = self.memory[mem].rotate_left(1);
		self.memory[mem] = new_value;
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RL r: Rotate r left through carry
	fn opcode_rl_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let c = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let mut new_value = self.cpu_registers[r_idx].rotate_left(1);
		new_value |= c;
		self.cpu_registers[r_idx] = new_value;
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RL m: Rotate m left through carry
	fn opcode_rl_m(&mut self, dreg_str: &str) {
		let mem = self.double_register_value(dreg_str);
		let c = (self.memory[mem] & 0x80) >> 7;
		let mut new_value = self.memory[mem].rotate_left(1);
		new_value |= c;
		self.memory[mem] = new_value;
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RRC r: Rotate register right 
	fn opcode_rrc_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let c = self.cpu_registers[r_idx] & 0x01;
		let new_value = self.cpu_registers[r_idx].rotate_right(1);
		self.cpu_registers[r_idx] = new_value;

		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RRC m: Rotate memory right
	fn opcode_rrc_m(&mut self, dreg_str: &str) {
		let mem = self.double_register_value(dreg_str);
		let c = self.memory[mem] & 0x01;
		let new_value = self.memory[mem].rotate_right(1);
		self.memory[mem] = new_value;
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RR r: Rotate register right through carry
	fn opcode_rr_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let c = self.cpu_registers[r_idx] & 0x01;
		let mut new_value = self.cpu_registers[r_idx].rotate_right(1);
		new_value |= c << 7;
		self.cpu_registers[r_idx] = new_value;

		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// RR m: Rotate memory right through carry
	fn opcode_rr_m(&mut self, dreg_str: &str) {
		let mem = self.double_register_value(dreg_str);
		let c = self.memory[mem] & 0x01;
		let mut new_value = self.memory[mem].rotate_right(1);
		new_value |= c << 7;
		self.memory[mem] = new_value;
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
	}

	// SLA r: Shift left register
	fn opcode_sla_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_idx].overflowing_shl(1);
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SLA m: Shift left memory pointed by double register
	fn opcode_sla_m(&mut self, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let (result, overflow) = self.memory[mem].overflowing_shl(1);
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SWAP r: Swap nibbles of register
	fn opcode_swap_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let value = self.cpu_registers[r_idx];
		let low_nibble = value & 0xF;
		let high_nibble = value & 0xF0;
		let new_value = (low_nibble << 4) | high_nibble;
		self.cpu_registers[r_idx] = new_value;

		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// SWAP m: Swap nibbles of memory pointed by double register
	fn opcode_swap_m(&mut self, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let value = self.memory[mem];
		let low_nibble = value & 0xF;
		let high_nibble = value & 0xF0;
		let new_value = (low_nibble << 4) | high_nibble;
		self.memory[mem] = new_value;

		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
	}

	// SRA r: Shift right arithmetic register (b7 = b7)
	fn opcode_sra_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let bit7 = self.cpu_registers[r_idx] & 0x80;
		let (mut result, overflow) = self.cpu_registers[r_idx].overflowing_shr(1);
		result |= bit7;
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SRA m: Shift right arithmetic memory pointed by double register (b7 = b7)
	fn opcode_sra_m(&mut self, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let bit7 = self.memory[mem] & 0x80;
		let (mut result, overflow) = self.memory[mem].overflowing_shr(1);
		result |= bit7;
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SRL r: Shift right logical register (b7 = 0)
	fn opcode_srl_r(&mut self, r: char) {
		let r_idx = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_idx].overflowing_shr(1);
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// SRL m: Shift right logical memory pointed by double register (b7 = 0)
	fn opcode_srl_m(&mut self, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let (result, overflow) = self.memory[mem].overflowing_shr(1);
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	}

	// BIT n, r: Test bit n in register r
	fn opcode_bit_nr(&mut self, n: u8, r: char) {
		let r_idx = self.r_index(r);
		let bit = (self.cpu_registers[r_idx] >> n) & 0x1;

		if bit == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
	}

	// BIT n, m: Test bit n in memory pointed by double register
	fn opcode_bit_nm(&mut self, n: u8, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let bit = (self.memory[mem] >> n) & 0x1;

		if bit == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
	}

	// Set n, r: Set bit n in register
	fn opcode_set_nr(&mut self, n: u8, r: char) {
		let r_idx = self.r_index(r);
		self.cpu_registers[r_idx] |= 1 << n;
	}

	// Set n, m: Set bit n in memory pointed by double register
	fn opcode_set_nm(&mut self, n: u8, dreg: &str) {
		let mem = self.double_register_value(dreg);
		self.memory[mem] |= 1 << n;
	}

	// Res n, r: Reset bit n in register
	fn opcode_res_nr(&mut self, n: u8, r: char) {
		let r_idx = self.r_index(r);
		self.cpu_registers[r_idx] &= !(1 << n);
	}
	
	// Res n, m: Reset bit n in memory pointed by double register
	fn opcode_res_nm(&mut self, n: u8, dreg: &str) {
		let mem = self.double_register_value(dreg);
		self.memory[mem] &= !(1 << n);
	}

	// CCF: Complement c flag, reset n and h flags
	fn opcode_ccf(&mut self) {
		let flag = self.get_flag('c');
		self.set_flag('c', flag ^ 1);

		self.set_flag('n', 0);
		self.set_flag('h', 0);
	}

	// SCF: Set c flag, reset n and h flags
	fn opcode_scf(&mut self) {
		self.set_flag('c', 1);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
	}

	// NOP: No operation
	fn opcode_nop(&mut self) {
		return;
	}

	// HALT: Halt in low pwer until interrupt occurs
	// TODO
	fn opcode_halt(&mut self) {
		
	}

	// STOP: Low power standby mode
	// TODO
	fn opcode_stop(&mut self) {
		
	}

	// DI: Disable interrupts
	fn opcode_di(&mut self) {
		self.ime = 0;
	}

	// EI: Enable interrupts
	// TODO: It is delayed by one instruction, fix later
	fn opcode_ei(&mut self) {
		self.ime = 1;
	}

	// JP: Jump to nn
	fn opcode_jp_nn(&mut self, nn: u16) {
		self.pc = nn;
	}

	// JP: Jump to memory pointed by double register
	fn opcode_jp_m(&mut self, dreg: &str) {
		self.pc = (self.double_register_value(dreg)) as u16;
	}

	// JP cc, nn: Jump conditional to nn
	fn opcode_jp_ccnn(&mut self, cc: u8, nn: u16) {
		let bits = cc & 0x03;
		let condition = match bits {
			0 => self.get_flag('z') == 0,
			1 => self.get_flag('z') == 1,
			2 => self.get_flag('c') == 0,
			3 => self.get_flag('c')== 1,
			_ => panic!("JP cc, nn"),
		};

		if condition == true {
			self.pc = nn;
		}
	}

	// JR dd: Relative jump to dd (signed)
	fn opcode_jr_dd(&mut self, dd: u8) {
		let signed_value = dd as i8;
		self.pc = self.pc.wrapping_add_signed(signed_value as i16);
	}

	// JR cc, dd: Relative jump to dd (signed) if condition is met
	fn opcode_jr_ccdd(&mut self, cc: u8, dd: u8) {
		let bits = cc & 0x03;
		let signed_value = dd as i8;
		let condition = match bits {
			0 => self.get_flag('z') == 0,
			1 => self.get_flag('z') == 1,
			2 => self.get_flag('c') == 0,
			3 => self.get_flag('c')== 1,
			_ => panic!("JR cc, dd"),
		};

		if condition == true {
			self.pc = self.pc.wrapping_add_signed(signed_value as i16);
		}
	}

	// CALL nn: Call subroutine at nn
	// TODO: Check the PC+2
	fn opcode_call_nn(&mut self, nn: u16) {
		self.push_stack(self.pc+2);
		self.pc = nn;
	}

	// CALL cc, nn: Call subroutine at nn if condition is met
	// TODO: Check the PC+2
	fn opcode_call_ccnn(&mut self, cc: u8, nn: u16) {
		let bits = cc & 0x03;
		let condition = match bits {
			0 => self.get_flag('z') == 0,
			1 => self.get_flag('z') == 1,
			2 => self.get_flag('c') == 0,
			3 => self.get_flag('c')== 1,
			_ => panic!("CALL cc, nn"),
		};

		if condition == true {
			self.push_stack(self.pc+2);
			self.pc = nn;
		}
	}

	// RET: Return from subroutine
	fn opcode_ret(&mut self) {
		self.pc = self.pop_stack();
	}

	// RET cc: Retrun from subroutine if condition is met
	fn opcode_ret_cc(&mut self, cc: u8) {
		let bits = cc & 0x03;
		let condition = match bits {
			0 => self.get_flag('z') == 0,
			1 => self.get_flag('z') == 1,
			2 => self.get_flag('c') == 0,
			3 => self.get_flag('c')== 1,
			_ => panic!("CALL cc, nn"),
		};

		if condition == true {
			self.pc = self.pop_stack();
		}
	}

	// RETI: Return and enable interrupts
	fn opcode_reti(&mut self) {
		self.ime = 1;
		self.pc = self.pop_stack();
	}

	// RST n: Call specific addresses
	// TODO: Check PC+2
	fn opcode_rst(&mut self, n: u8) {
		self.push_stack(self.pc+2);
		self.pc = n as u16;
	}
}
