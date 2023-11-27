use std::fs::File;
use std::io::Read;

use crate::bus::Bus;
use crate::cpu::CPU;
use crate::color::Color;
use crate::ppu::GB_WIDTH;
use crate::ppu::GB_HEIGHT;

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

	// Run instructions until we are ready to display a new frame
	pub fn run(&mut self) -> &[[Color; GB_WIDTH]; GB_HEIGHT] {
		while self.bus.ppu.frame_ready == false {
			self.cpu.step(&mut self.bus);			
		}
		self.bus.ppu.frame_ready = false;
		self.bus.ppu.get_screen_buffer()
	}

	pub fn get_tilemap(&self) -> [[[Color; 8]; 8]; 384] {
		self.bus.ppu.get_tilemap()
	}
}

#[cfg(test)]
mod tests;
