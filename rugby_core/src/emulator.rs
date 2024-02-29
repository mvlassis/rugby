use std::collections::VecDeque;
use std::process;
use std::path::PathBuf;
use std::time::Duration;

use crate::bus::Bus;
use crate::cartridge::load;
use crate::cpu::CPU;
use crate::color::Color;
use crate::color::LogicalColor;
use crate::input::Input;
use crate::input::EmulatorInput;
use crate::ppu::GB_WIDTH;
use crate::ppu::GB_HEIGHT;
use crate::save_state::EmulatorState;

const REWIND_STACK_CAPACITY: usize = 300; // 60 equals about 1 second
const REWIND_TIME: u64 = 5; 

pub struct Emulator {
	cpu: CPU,
	bus: Bus,

	rewind_stack: VecDeque<String>,
	rewind_screens: VecDeque<[[Color; GB_WIDTH]; GB_HEIGHT]>,
	pub save_states: Vec<String>,
	pub select_save_states: Vec<String>,

	emulator_active: bool,
	current_bg_map: u8, // The background map to show (for debugging)
}

impl Emulator {
	pub fn new(data_buffer: Option<Vec<u8>>, path_buf: Option<PathBuf>, callback: Box<dyn Fn(&[f32])>) -> Self {
		let emulator_active = path_buf.is_some();
		let (cartridge, gb_mode) = load(data_buffer, path_buf);
		
		let mut bus = Bus::new(cartridge, callback);
		bus.initialize(gb_mode);
		
		let mut cpu = CPU::new();
		cpu.initialize(gb_mode);
		
		Emulator {
			cpu,
			bus,

			rewind_stack: VecDeque::with_capacity(REWIND_STACK_CAPACITY),
			rewind_screens: VecDeque::with_capacity(REWIND_STACK_CAPACITY),
			save_states: Vec::new(),
			select_save_states: vec!["".to_string(); 4],
			emulator_active,
			current_bg_map: 0
		}
	}

	// Loads a new ROM file
	pub fn load(&mut self, data_buffer: Option<Vec<u8>>, path_buf: Option<PathBuf>) {
		self.emulator_active = true;
		let (cartridge, gb_mode) = load(data_buffer, path_buf);
		self.bus.load_rom(cartridge);
		self.bus.initialize(gb_mode);

		self.cpu.initialize(gb_mode);

		self.save_states = Vec::new();
		self.select_save_states = vec!["".to_string(); 4];
	}

	// Run instructions until we are ready to display a new frame
	pub fn run(&mut self, input: Input, emulator_input: Option<EmulatorInput>) -> &[[Color; GB_WIDTH]; GB_HEIGHT] {
		if let Some(emu_input) = emulator_input {
			self.update_config(emu_input);
		}

		// If emulator is not active, simply return a blank screen
		if !self.emulator_active {
			return &[[Color::Logical(LogicalColor::White); GB_WIDTH]; GB_HEIGHT];
		}

		// Rewind
		if emulator_input.is_some() && emulator_input.unwrap().rewind {
			#[cfg(feature = "rewind")]
			{
				self.pop_rewind_stack();
				self.bus.ppu.frame_ready = false;
				std::thread::sleep(Duration::from_millis(REWIND_TIME));
				return self.bus.ppu.get_screen_buffer();
			}
		}
		
		self.bus.mmu.store_input(input);
		while self.bus.ppu.frame_ready == false {
			self.cpu.step(&mut self.bus);			
		}
		{
			
		}
		#[cfg(feature = "rewind")]
		{
			self.push_rewind_stack();
		}
		self.bus.ppu.frame_ready = false;
		self.bus.ppu.get_screen_buffer()
	}

	pub fn get_screen(&self) -> &[[Color; GB_WIDTH]; GB_HEIGHT] {
		self.bus.ppu.get_screen_buffer()
	}

	// Updates the emulator's settings
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
		for i in 0..=2 {
			if emulator_input.toggle_layer[i] {
				self.bus.ppu.toggle_layer(i);
			}
		}
		for i in 0..=3 {
			if emulator_input.toggle_channel[i] {
				self.bus.apu.toggle_channel(i);
			}
		}
	}

	// Save the cartridge
	pub fn save(&mut self) {
		self.bus.mmu.cartridge.save();
	}
	
	// Creates an EmulatorState from the currently running Emulator
	pub fn save_state(&mut self, position: Option<usize>) {
		let save_string = self.get_save_string();
		match position {
			Some(i) => self.select_save_states[i] = save_string,
			None => self.save_states.push(save_string), 
		}
	}

	fn get_save_string(&mut self) -> String {
		let json = self.bus.mmu.cartridge.create_state();
		let emulator_state = EmulatorState {
			cpu_state: self.cpu.create_state(),
			bus_state: self.bus.create_state(),
			cartridge_json: json,
		};
		let serialized = serde_json::to_string(&emulator_state).unwrap();
		// println!("Size of JSON: {} bytes", serialized.len());
		serialized
	}

	// Loads an EmulatorState from either the save stack or the 4 save states
	pub fn load_state(&mut self, position: Option<usize>) {
		match position {
			Some(i) => {
				if self.select_save_states[i] != "" {
					self.load_save_string(i as i8);
				}
			}
			None => {
				if let Some(_) = self.save_states.last() {
					self.load_save_string(5);
				}
			}
		}
		
	}

	// Load an Emulator state based on the option given
	fn load_save_string(&mut self, option: i8) {
		let state_string = match option {
			-1 => &self.rewind_stack.back().unwrap(),
			0..=3 => &self.select_save_states[option as usize],
			_ => &self.save_states.last().unwrap(),
		};
		match serde_json::from_str::<EmulatorState>(&state_string) {
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
	
	pub fn push_rewind_stack(&mut self) {
		if self.rewind_stack.len() == REWIND_STACK_CAPACITY {
			self.rewind_stack.pop_front();
			self.rewind_screens.pop_front();
		}
		let state = self.get_save_string();
		self.rewind_stack.push_back(state);
		self.rewind_screens.push_back(self.get_screen().clone());
	}

	pub fn pop_rewind_stack(&mut self) {
		if self.rewind_stack.len() > 0 {
			self.load_save_string(-1);
			self.bus.ppu.screen_buffer = self.rewind_screens.pop_back().unwrap();
			self.rewind_stack.pop_back();
		}
	}
	
	pub fn get_tilemap(&self) -> [[[Color; 8]; 8]; 384] {
		self.bus.ppu.get_tilemap()
	}

	pub fn get_bg_map(&self) -> [[[Color; 8]; 8]; 1024] {
		self.bus.ppu.get_bg_map(self.current_bg_map)
	}
}

