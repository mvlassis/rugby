use std::fs::File;
use std::io::Read;

use crate::bus::Bus;
use crate::cpu::CPU;
use crate::color::Color;
use crate::input::Input;
use crate::input::EmulatorInput;
use crate::ppu::GB_WIDTH;
use crate::ppu::GB_HEIGHT;

pub struct Emulator {
	cpu: CPU,
	bus: Bus,

	current_bg_map: u8, // Show the background maps (for debugging)
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

			current_bg_map: 0
		}
	}

	pub fn load(&mut self, path: &str) {
		let mut rom = File::open(path).expect("Unable to open file {path}");
		let mut data_buffer = Vec::new();
		rom.read_to_end(&mut data_buffer).unwrap();
		self.bus.mmu.load(&data_buffer);
	}

	// Run instructions until we are ready to display a new frame
	pub fn run(&mut self, input: Input, emulator_input: Option<EmulatorInput>) -> &[[Color; GB_WIDTH]; GB_HEIGHT] {
		if let Some(emulator_input) = emulator_input {
			self.update_config(emulator_input);
		}
		self.bus.mmu.store_input(input);
		while self.bus.ppu.frame_ready == false {
			self.cpu.step(&mut self.bus);			
		}
		self.bus.ppu.frame_ready = false;
		self.bus.ppu.get_screen_buffer()
	}

	pub fn update_config(&mut self, emulator_input: EmulatorInput) {
		if emulator_input.prev_bg_map {
			self.current_bg_map = (self.current_bg_map + 1) % 4
		}
		if emulator_input.next_bg_map {
			self.current_bg_map  = if self.current_bg_map == 0 {3} else {self.current_bg_map - 1};
		}
	}
	
	pub fn get_tilemap(&self) -> [[[Color; 8]; 8]; 384] {
		self.bus.ppu.get_tilemap()
	}

	pub fn get_bg_map(&self) -> [[[Color; 8]; 8]; 1024] {
		self.bus.ppu.get_bg_map(self.current_bg_map)
	}
}

#[cfg(test)]
mod tests;
