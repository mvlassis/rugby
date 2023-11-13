mod cpu;
mod mmu;

use std::fs::File;
use std::io::Read;

use crate::cpu::CPU;
use crate::mmu::MMU;

pub struct GameBoy {
	cpu: CPU,
	mmu: MMU,
}

impl GameBoy {
	pub fn new() -> Self {
		GameBoy {
			cpu: CPU::new(),
			memory: MMU::new(),
		}
	}

	pub fn load(&mut self, path: &str) {
		let rom = File::open(path).expect("Unable to open file {path}");
		let mut data_buffer = Vec::new();
		rom.read_to_end(&mut data_buffer).unwrap();
		self.mmu.load(&data_buffer);
	}
	
	pub fn run(&mut self) {
		;
	}
}

#[cfg(test)]
mod tests;
