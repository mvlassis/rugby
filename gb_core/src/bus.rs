use crate::apu::APU;
use crate::cartridge::Cartridge;
use crate::mmu::MMU;
use crate::ppu::PPU;

pub struct Bus {
	pub apu: APU,
	pub mmu: MMU,
	pub ppu: PPU,

	dma_active: bool,
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
			dma_active: false,
		}
	}

	pub fn initialize(&mut self) {
		self.mmu.initialize();
		self.ppu.initialize();
		self.dma_active = false;
	}
	
	pub fn tick(&mut self) {
		for _ in 0..4 {
			self.apu.tick(self.mmu.timer.div);
		}
		for _ in 0..4 {
			self.ppu.dot();
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
			},
			0xFF47 => self.ppu.bgp = value,
			0xFF48 => self.ppu.obp0 = value,
			0xFF49 => self.ppu.obp1 = value,
			0xFF4A => self.ppu.wy = value,
			0xFF4B => self.ppu.wx = value,
			_ => self.mmu.set_byte(address, value),
		}
	}

	// DMA transfer
	// TODO: Make it timing correct
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
}
