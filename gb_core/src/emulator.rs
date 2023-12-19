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
use crate::save_state::EmulatorState;

pub struct Emulator {
	cpu: CPU,
	bus: Bus,

	pub save_states: Vec<String>,
	pub select_save_states: Vec<String>,
	
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

			save_states: Vec::new(),
			select_save_states: vec!["".to_string(); 4],
			current_bg_map: 0
		}
	}

	// Loads a new ROM file
	pub fn load(&mut self, path_buf: PathBuf) {
		self.cpu.initialize();
		let cartridge = load(path_buf);
		self.bus.load_rom(cartridge);
		self.bus.initialize();

		self.save_states = Vec::new();
		self.select_save_states = vec!["".to_string(); 4];
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

	pub fn get_screen(&self) -> &[[Color; GB_WIDTH]; GB_HEIGHT] {
		self.bus.ppu.get_screen_buffer()
	}

	// Changes the emulator's settings
	pub fn update_config(&mut self, emulator_input: EmulatorInput) {
		if emulator_input.prev_bg_map {
			self.current_bg_map = (self.current_bg_map + 1) % 4
		}
		if emulator_input.next_bg_map {
			self.current_bg_map  = if self.current_bg_map == 0 {3} else {self.current_bg_map - 1};
		}
		if emulator_input.exit {
			self.bus.mmu.cartridge.save();
			process::exit(0);
		}
		if emulator_input.save_state {
			self.save_state(None);
		}
		if emulator_input.load_state {
			self.load_state(None);
		}
		if emulator_input.select_save_state.0 {
			self.save_state(Some(emulator_input.select_save_state.1));
		}
		if emulator_input.select_load_state.0 {
			self.load_state(Some(emulator_input.select_load_state.1));
		}
		if emulator_input.toggle_mute {
			self.bus.apu.toggle_mute();
		}
		for i in 0..=3 {
			if emulator_input.toggle_channel[i] {
				self.bus.apu.toggle_channel(i);
			}
		}
	}

	// Creates an EmulatorState from the currently running Emulator
	pub fn save_state(&mut self, position: Option<usize>) {
		let json = self.bus.mmu.cartridge.create_state();
		let emulator_state = EmulatorState {
			cpu_state: self.cpu.create_state(),
			bus_state: self.bus.create_state(),
			cartridge_json: json,
		};
		let serialized = serde_json::to_string(&emulator_state).unwrap();
		// println!("Size of JSON: {} bytes", serialized.len());
		match position {
			Some(i) => self.select_save_states[i] = serialized,
			None => self.save_states.push(serialized), 
		}

	}

	// Loads an EmulatorState from either the save stack or the 4 save states
	pub fn load_state(&mut self, position: Option<usize>) {
		match position {
			Some(i) => {
				if self.select_save_states[i] != "" {
					match serde_json::from_str::<EmulatorState>(&self.select_save_states[i]) {
						Ok(emulator_state) => {
							self.cpu.load_state(emulator_state.cpu_state);
							self.bus.load_state(emulator_state.bus_state);
							self.bus.mmu.cartridge.load_state(&emulator_state.cartridge_json);
						},
						Err(e) => {
							eprintln!("Failed to deserialize state: {}", e);
						}
					}
				}
			},
			None => {
				if let Some(last_state_json) = self.save_states.last() {
					match serde_json::from_str::<EmulatorState>(last_state_json) {
						Ok(emulator_state) => {
							self.cpu.load_state(emulator_state.cpu_state);
							self.bus.load_state(emulator_state.bus_state);
							self.bus.mmu.cartridge.load_state(&emulator_state.cartridge_json);
						},
						Err(e) => {
							eprintln!("Failed to deserialize state: {}", e);
						}
					}
				}
			}
		}
		
	}
	
	pub fn get_tilemap(&self) -> [[[Color; 8]; 8]; 384] {
		self.bus.ppu.get_tilemap()
	}

	pub fn get_bg_map(&self) -> [[[Color; 8]; 8]; 1024] {
		self.bus.ppu.get_bg_map(self.current_bg_map)
	}
}

