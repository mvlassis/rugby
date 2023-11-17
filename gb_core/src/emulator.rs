use std::fs::File;
use std::io::Read;

use crate::cpu::CPU;
use crate::mmu::MMU;

pub struct Emulator {
	cpu: CPU,
	mmu: MMU,
}

impl Emulator {
	pub fn new() -> Self {
		let mut cpu = CPU::new();
		cpu.initialize();
		Emulator {
			cpu,
			mmu: MMU::new(),
		}
	}

	pub fn load(&mut self, path: &str) {
		let mut rom = File::open(path).expect("Unable to open file {path}");
		let mut data_buffer = Vec::new();
		rom.read_to_end(&mut data_buffer).unwrap();
		self.mmu.load(&data_buffer);
	}
	
	pub fn run(&mut self) {
		self.cpu.step(&mut self.mmu);
	}
}

#[cfg(test)]
mod tests;
