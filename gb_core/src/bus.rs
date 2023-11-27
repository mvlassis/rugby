use crate::mmu::MMU;
use crate::ppu::PPU;

pub struct Bus {
	pub mmu: MMU,
	pub ppu: PPU,
}

impl Bus {
	pub fn new() -> Self {
		let mmu = MMU::new();
		let ppu = PPU::new();
		Bus {
			mmu,
			ppu,
		}
	}

	pub fn initialize(&mut self) {
		self.mmu.initialize();
		self.ppu.initialize();
	}
	
	pub fn tick(&mut self) {
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
			0xFF41 => self.ppu.stat,
			0xFF42 => self.ppu.scy,
			0xFF43 => self.ppu.scx,
			0xFF44 => self.ppu.ly,
			0xFF45 => self.ppu.lyc,
			0xFF47 => self.ppu.bgp,
			_ => self.mmu.get_byte(address),
		}
	}

	// Set an 8-bit value at a specific address in memory
	pub fn set_byte(&mut self, address: u16, value: u8) {
		match address {
			0x8000..=0x9FFF => self.ppu.set_vram(address as usize - 0x8000, value),
			0xFF41 => self.ppu.stat = value,
			0xFF42 => self.ppu.scy = value,
			0xFF43 => self.ppu.scx = value,
			0xFF45 => self.ppu.lyc = value,
			0xFF47 => self.ppu.bgp = value,
			_ => self.mmu.set_byte(address, value),
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
