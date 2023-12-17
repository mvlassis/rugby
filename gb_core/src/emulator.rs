use std::process;
use std::path::PathBuf;

use crate::bus::Bus;
use crate::cartridge::load;
use crate::cpu::CPU;
use crate::color::Color;
use crate::input::Input;
use crate::input::EmulatorInput;
use crate::ppu::GB_WIDTH;
use crate::ppu::GB_HEIGHT;

pub struct Emulator {
	cpu: CPU,
	bus: Bus,

	current_bg_map: u8, // The background map to show (for debugging)
}

impl Emulator {
	pub fn new(path_buf: PathBuf, callback: Box<dyn Fn(&[f32])>) -> Self {
		let mut cpu = CPU::new();
		cpu.initialize();

		let cartridge = load(path_buf);
		let mut bus = Bus::new(cartridge, callback);

		bus.initialize();
		Emulator {
			cpu,
			bus,

			current_bg_map: 0
		}
	}

	pub fn load(&mut self, path_buf: PathBuf) {
		self.cpu.initialize();
		let cartridge = load(path_buf);
		self.bus.load_rom(cartridge);
		self.bus.initialize();
	}

	// Run instructions until we are ready to display a new frame
	pub fn run(&mut self, input: Input, emulator_input: Option<EmulatorInput>) -> &[[Color; GB_WIDTH]; GB_HEIGHT] {
		if let Some(emulator_input) = emulator_input {
			if emulator_input.exit == true {
				self.bus.mmu.cartridge.save();
				process::exit(0);
			}
			if emulator_input.toggle_mute == true {
				self.bus.apu.toggle_mute();
			}
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

