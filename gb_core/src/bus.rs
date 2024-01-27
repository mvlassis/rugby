use crate::apu::APU;
use crate::cartridge::Cartridge;
use crate::gb_mode::GBMode;
use crate::mmu::MMU;
use crate::ppu::PPU;
use crate::save_state::BusState;

struct HDMA {
	hdma_index: usize,
	hdma_length: usize,
	hdma_active: bool,
	src_address: u16,
	dest_address: u16,	
}

pub struct Bus {
	pub apu: APU,
	pub mmu: MMU,
	pub ppu: PPU,

	// For Gameboy Color
	gb_mode: GBMode,
	hdma: [u8; 5],
	dma_active: bool,
	hdma_struct: HDMA,
	pub key1: u8,
	pub double_speed: bool,
}

impl Bus {
	pub fn new(cartridge: Box<dyn Cartridge>, callback: Box<dyn Fn(&[f32])>) -> Self {
		let apu = APU::new(callback);
		let mmu = MMU::new(cartridge);
		let ppu = PPU::new();
		Bus {
			apu,
			mmu,
			ppu,

			gb_mode: GBMode::DMG,
			hdma: [0xFF; 5],
			dma_active: false,
			hdma_struct: HDMA {
				hdma_index: 0,
				hdma_length: 0,
				hdma_active: false,
				src_address: 0,
				dest_address: 0,
			},
			key1: 0x7E,
			double_speed: false,
		}
	}

	// Initializes Bus
	pub fn initialize(&mut self, gb_mode: GBMode) {
		self.gb_mode = gb_mode;
		
		self.mmu.initialize(gb_mode);
		self.ppu.initialize(gb_mode);
		self.apu.reset();
		
		self.dma_active = false;
		self.hdma = [0xFF; 5];
		self.hdma_struct = HDMA {
			hdma_index: 0,
			hdma_length: 0,
			hdma_active: false,
			src_address: 0,
			dest_address: 0,
		};
		self.key1 = 0x7E;
		self.double_speed = false;
	}
	
	pub fn load_rom(&mut self, cartridge: Box<dyn Cartridge>) {
		self.mmu = MMU::new(cartridge);
	}
	
	pub fn tick(&mut self) {
		let speed_factor = if self.double_speed {2} else {1};
		for _ in 0..(4 / speed_factor) {
			self.apu.tick(self.mmu.timer.div);
		}
		for _ in 0..(4 / speed_factor) {
			self.ppu.dot();
		}
		// Check for HDMA transfers
		if self.ppu.entered_hblank {
			self.ppu.entered_hblank = false;
			if self.hdma_struct.hdma_active {
				// Transfer a block of 16 bytes
				for i in 0..16 {
					let src_index = self.hdma_struct.src_address + self.hdma_struct.hdma_index as u16 + i;
					let dest_index = self.hdma_struct.dest_address + self.hdma_struct.hdma_index as u16 + i;
					let value = self.mmu.get_byte(src_index);
					self.ppu.set_vram(dest_index as usize, value);
				}
				self.hdma_struct.hdma_index += 16;
				if self.hdma_struct.hdma_index >= self.hdma_struct.hdma_length {
					self.hdma_struct.hdma_active = false;
					self.hdma[4] = 0xFF;
				}
			}
		}
		
		if self.ppu.vblank_interrupt == true {
			self.ppu.vblank_interrupt = false;
			let mut if_register = self.mmu.get_byte(0xFF0F);
			if_register = Bus::set_bit(if_register, 0, 1);
			self.mmu.set_byte(0xFF0F, if_register);
		}
		if self.ppu.stat_interrupt == true {
			self.ppu.stat_interrupt = false;
			let mut if_register = self.mmu.get_byte(0xFF0F);
			if_register = Bus::set_bit(if_register, 1, 1);
			self.mmu.set_byte(0xFF0F, if_register);
		}
		self.mmu.timer.tick();
		if self.mmu.timer.timer_interrupt == 1 {
			self.mmu.timer.timer_interrupt = 0;
			let mut if_register = self.mmu.get_byte(0xFF0F);
			if_register = Bus::set_bit(if_register, 2, 1);
			self.mmu.set_byte(0xFF0F, if_register);
		}
		if self.mmu.joypad_interrupt == true {
			self.mmu.joypad_interrupt = false;
			let mut if_register = self.mmu.get_byte(0xFF0F);
			if_register = Bus::set_bit(if_register, 4, 1);
			self.mmu.set_byte(0xFF0F, if_register);
		}
	}

	// Get 8-bit value from memory at a specific address
	pub fn get_byte(&self, address: u16) -> u8 {
		match address {
			0x8000..=0x9FFF => self.ppu.get_vram(address as usize - 0x8000),
			0xFE00..=0xFE9F => self.ppu.get_oam(address as usize - 0xFE00),
			0xFF10..=0xFF3F => self.apu.get_byte(address),
			0xFF40 => self.ppu.lcdc,
			0xFF41 => self.ppu.stat,
			0xFF42 => self.ppu.scy,
			0xFF43 => self.ppu.scx,
			0xFF44 => self.ppu.ly,
			0xFF45 => self.ppu.lyc,
			0xFF47 => self.ppu.bgp,
			0xFF48 => self.ppu.obp0,
			0xFF49 => self.ppu.obp1,
			0xFF4A => self.ppu.wy,
			0xFF4B => self.ppu.wx,
			0xFF4D => match self.gb_mode {
				GBMode::DMG => 0xFF,
				GBMode::CGB => match self.double_speed {
						true => 0xFE | (self.key1 & 0x01),
						false => 0x7E | (self.key1 & 0x01)
				},
			},
			0xFF4F => self.ppu.vbk | 0xFE,
			0xFF51..=0xFF54 => match self.gb_mode {
				GBMode::DMG => 0xFF,
				GBMode::CGB => self.hdma[address as usize - 0xFF51],
			},
			0xFF55 => match self.gb_mode {
				GBMode::DMG => 0xFF,
				GBMode::CGB => match self.hdma_struct.hdma_active {
					true => {
						let value = (self.hdma_struct.hdma_length / 0x10) - 1;
						value as u8
					},
					false => 0xFF,
				},
			}
			0xFF68 => match self.ppu.gb_mode {
				GBMode::DMG => 0xFF,
				GBMode::CGB => self.ppu.bgpi as u8,
			},
			0xFF69 => match self.ppu.gb_mode {
				GBMode::DMG => 0xFF,
				GBMode::CGB => self.ppu.bgpi as u8,
			},
			0xFF6A => match self.ppu.gb_mode {
				GBMode::DMG => 0xFF,
				GBMode::CGB => self.ppu.obpi as u8,
			},
			0xFF6B => match self.ppu.gb_mode {
				GBMode::DMG => 0xFF,
				GBMode::CGB => self.ppu.bgpi as u8,
			},
			_ => self.mmu.get_byte(address),
		}
	}

	// Get 16-bit value from memory at a specific address
	pub fn get_word(&self, address: u16) -> u16 {
		let byte1 = self.get_byte(address) as u16;
		let byte2 = self.get_byte(address+1) as u16;
		(byte2 << 8) | byte1
	}
	
	// Set an 8-bit value at a specific address in memory
	pub fn set_byte(&mut self, address: u16, value: u8) {
		match address {
			0x8000..=0x9FFF => self.ppu.set_vram(address as usize - 0x8000, value),
			0xFE00..=0xFE9F => self.ppu.set_oam(address as usize - 0xFE00, value),
			0xFF10..=0xFF3F => self.apu.set_byte(address, value),
			0xFF40 => self.ppu.lcdc = value,
			0xFF41 => self.ppu.stat = value,
			0xFF42 => self.ppu.scy = value,
			0xFF43 => self.ppu.scx = value,
			0xFF45 => self.ppu.lyc = value,
			0xFF46 => {
				self.dma_active = true;
				self.dma_transfer(value);
				self.dma_active = false;
			},
			0xFF47 => self.ppu.bgp = value,
			0xFF48 => self.ppu.obp0 = value,
			0xFF49 => self.ppu.obp1 = value,
			0xFF4A => self.ppu.wy = value,
			0xFF4B => self.ppu.wx = value,
			0xFF4D => self.key1 = value,
			0xFF4F => self.ppu.vbk = value & 0x01,
			0xFF51..=0xFF54 => match self.gb_mode {
				GBMode::DMG => (),
				GBMode::CGB => self.hdma[address as usize - 0xFF51] = value,
			},
			0xFF55 => match self.gb_mode {
				GBMode::DMG => (),
				GBMode::CGB => {
					self.hdma[4] = value;
					let dma_length = ((self.hdma[4] & 0x7F) as u16 + 1) * 0x10;
					let dma_mode = self.hdma[4] & 0x80;
					if self.hdma_struct.hdma_active && dma_mode == 0 {
						self.hdma_struct.hdma_active = false;
					} else {
						let high_byte = self.hdma[0];
						let low_byte = self.hdma[1];
						let src_address = ((high_byte as u16) << 8) | (low_byte as u16 & 0xF0);

						let high_byte = self.hdma[2];
						let low_byte = self.hdma[3];
						let dest_address = ((high_byte as u16 & 0x1F) << 8) | (low_byte as u16 & 0xF0);
						
						if dma_mode != 0 {
							self.hdma_struct.hdma_length = dma_length as usize;
							self.hdma_struct.hdma_index = 0;
							self.hdma_struct.hdma_active = true;
							self.hdma_struct.src_address = src_address;
							self.hdma_struct.dest_address = dest_address;
						} else {
							// General-purpose DMA
							for i in 0..dma_length {
								let value = self.mmu.get_byte(src_address + i);
								self.ppu.set_vram(dest_address as usize + i as usize, value);
							}
						}
					}

				},
			},
			0xFF68 => self.ppu.bgpi = value,
			0xFF6A => self.ppu.obpi = value,
			0xFF69 | 0xFF6B => self.ppu.set_palette(address as usize, value),
			_ => self.mmu.set_byte(address, value),
		}
	}

	// DMA transfer
	// TODO: Make the timing correct
	fn dma_transfer(&mut self, value: u8) {
		let base_address = (value as u16) << 8;
		for i in 0..=0x9F {
			let byte = self.get_byte(base_address + i);
			self.set_byte(0xFE00+i, byte);
		}
	}
		
	// Set a bit in a u8
	fn set_bit(value: u8, bit_position: u8, bit_value: u8) -> u8 {
		let new_value = match bit_value {
			0 => value & !(1 << bit_position),
			1 => value | 1 << bit_position,
			_ => panic!("Set bit"),
		};
		new_value
	}

	// Creates a BusState from the Bus
	pub fn create_state(&self) -> BusState {
		BusState {
			mmu_state: self.mmu.create_state(),
			ppu_state: self.ppu.create_state(),
			apu_state: self.apu.create_state(),
		}
	}

	// Loads a BusState to the Bus
	pub fn load_state(&mut self, bus_state: BusState) {
		self.mmu.load_state(bus_state.mmu_state);
		self.ppu.load_state(bus_state.ppu_state);
		self.apu.load_state(bus_state.apu_state);
	}
}
