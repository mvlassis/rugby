use std::fs::File;
use std::io::Read;

use crate::bus::Bus;
use crate::cpu::CPU;

pub struct Emulator {
	cpu: CPU,
	bus: Bus,
}

impl Emulator {
	pub fn new() -> Self {
		let mut cpu = CPU::new();
		cpu.initialize();
		let mut bus = Bus::new();
		bus.initialize();
		Emulator {
			cpu,
			bus,
		}
	}

	pub fn load(&mut self, path: &str) {
		let mut rom = File::open(path).expect("Unable to open file {path}");
		let mut data_buffer = Vec::new();
		rom.read_to_end(&mut data_buffer).unwrap();
		self.bus.mmu.load(&data_buffer);
	}
	
	pub fn run(&mut self) {
		self.cpu.step(&mut self.bus);
	}
}

#[cfg(test)]
mod tests;
