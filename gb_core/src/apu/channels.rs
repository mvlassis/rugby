

pub struct SoundRegisters {
	pub nrx0: u8, // Channel specific features
	pub nrx1: u8, // Length timer
	pub nrx2: u8, // Volume and envelope
	pub nrx3: u8, // Period
	pub nrx4: u8, // Trigger and length timer enable
}

impl SoundRegisters {
	pub fn clear(&mut self) {
		self.nrx0 = 0;
		self.nrx1 = 0;
		self.nrx2 = 0;
		self.nrx3 = 0;
		self.nrx4 = 0;
	}
}

#[derive(Clone, Copy)]
pub enum ChannelType {
	Pulse1,
	Pulse2,
}

pub struct PulseChannel {
	pub reg: SoundRegisters,
	pub channel: ChannelType,
	len_timer: u8,
	pub active: bool,
}

impl PulseChannel {
	pub fn new(channel: ChannelType) -> Self {
		let reg = match channel {
			ChannelType::Pulse1 => SoundRegisters {
				nrx0: 0x80,
				nrx1: 0xBF,
				nrx2: 0xF3,
				nrx3: 0xFF,
				nrx4: 0xBF,
			},
			ChannelType::Pulse2 => SoundRegisters {
				nrx0: 0, // Not used
				nrx1: 0x3F,
				nrx2: 0x00,
				nrx3: 0xFF,
				nrx4: 0xBF,
			}
		};
		PulseChannel {
			reg,
			channel,
			len_timer: 0,
			active: false,
		}
	}

	pub fn tick(&mut self, div_apu: u8) {
		if div_apu % 2 == 0 && self.length_enable() && self.len_timer > 0 {
			self.len_timer -= 1;
			if self.len_timer == 0 {
				self.active = false;
			}
		}
	}

	// Returns the status of the DAC
	pub fn dac_status(&self) -> bool {
		if self.reg.nrx2 & 0xF8 != 0 {
			true
		} else {
			false
		}
	}
	
	pub fn get_reg(&self, register: u8) -> u8 {
		match register {
			0 => match self.channel {
				ChannelType::Pulse1 => self.reg.nrx0 | 0x80,
				ChannelType::Pulse2 => 0xFF,
			}
			1 => self.reg.nrx1 | 0x3F,
			2 => self.reg.nrx2,
			3 => self.reg.nrx3 | 0xFF,
			4 => self.reg.nrx4 | 0xBF,
			_ => unreachable!("PulseChannel::get_reg()"),
		}
	}

	pub fn set_reg(&mut self, register: u8, value: u8) {
		match register {
			0 => self.reg.nrx0 = value,
			1 => {
				self.reg.nrx1 = value;
				let six_bits = value & 0x3F;
				self.len_timer = 64 - six_bits;
			},
			2 => {
				self.reg.nrx2 = value;
				if !self.dac_status() {
					self.active = false;
				}
			}
			3 => self.reg.nrx3 = value,
			4 => {
				self.reg.nrx4 = value;
				if (self.reg.nrx4 >> 7) & 0x1 == 1 {
					self.trigger();
				}
			}
			_ => unreachable!("PuseChannel::set_reg()"),
		};
	}

	pub fn clear(&mut self) {
		self.reg.clear();
		self.active = false;
	}

	fn trigger(&mut self) {
		if self.len_timer == 0 {
			self.len_timer = 64;
		}
		if self.dac_status() {
			self.active = true;
		}
	}
	
	fn length_enable(&self) -> bool {
		if (self.reg.nrx4 >> 6) & 0x01 == 1 {
			true
		} else {
			false
		}
	}
}

pub struct WaveChannel {
	pub reg: SoundRegisters,
	pub wave_pattern: [u8; 16],
	len_timer: u16,
	pub active: bool,
}

impl WaveChannel {
	pub fn new() -> Self {
		let reg = SoundRegisters {
			nrx0: 0x7F,
			nrx1: 0xFF,
			nrx2: 0x9F,
			nrx3: 0xFF,
			nrx4: 0xBF,
		};
		WaveChannel {
			reg,
			wave_pattern: [0; 16],
			len_timer: 0,
			active: false,
		}
	}

	pub fn tick(&mut self, div_apu: u8) {
		if div_apu % 2 == 0 && self.length_enable() && self.len_timer > 0 {
			self.len_timer -= 1;
			if self.len_timer == 0 {
				self.active = false;
			}
		}
	}
	
	pub fn get_reg(&self, register: u8) -> u8 {
		match register {
			0 => self.reg.nrx0 | 0x7F,
			1 => self.reg.nrx1 | 0xFF,
			2 => self.reg.nrx2 | 0x9F,
			3 => self.reg.nrx3 | 0xFF,
			4 => self.reg.nrx4 | 0xBF,
			_ => unreachable!("WaveChannel::get_reg()"),
		}
	}

	pub fn set_reg(&mut self, register: u8, value: u8) {
		match register {
			0 => {
				self.reg.nrx0 = value;
				if !self.dac_status() {
					self.active = false;
				}
			}
			1 => {
				self.reg.nrx1 = value;
				self.len_timer = 256 - (value as u16);
			}
			2 => self.reg.nrx2 = value,
			3 => self.reg.nrx3 = value,
			4 => {
				self.reg.nrx4 = value;
				if (self.reg.nrx4 >> 7) & 0x01 == 1 {
					self.trigger();
				}
			}
			_ => unreachable!("WaveChannel::get_reg()"),
		};
	}

	// Returns true if DAC is on, false if DAC is off
	pub fn dac_status(&self) -> bool {
		if (self.reg.nrx0 >> 7) & 0x1 == 1 {
			true
		} else {
			false
		}
	}

	pub fn clear(&mut self) {
		self.reg.clear();
		self.active = false;
	}

	fn trigger(&mut self) {
		if self.len_timer == 0 {
			self.len_timer = 256;
		}
		if self.dac_status() {
			self.active = true;
		}
	}
	
	fn length_enable(&self) -> bool {
		if (self.reg.nrx4 >> 6) & 0x01 == 1 {
			true
		} else {
			false
		}
	}
}

pub struct NoiseChannel {
	pub reg: SoundRegisters,
	len_timer: u8,
	pub active: bool,
}

impl NoiseChannel {
	pub fn new() -> Self {
		let reg = SoundRegisters {
			nrx0: 0, // Not used
			nrx1: 0xFF,
			nrx2: 0x00,
			nrx3: 0x00,
			nrx4: 0xBF,
		};
		NoiseChannel {
			reg,
			len_timer: 0,
			active: false,
		}
	}

	pub fn tick(&mut self, div_apu: u8) {
		if div_apu % 2 == 0 && self.length_enable() && self.len_timer > 0 {
			self.len_timer -= 1;
			if self.len_timer == 0 {
				self.active = false;
			}
		}
	}
	
	pub fn get_reg(&self, register: u8) -> u8 {
		match register {
			0 => self.reg.nrx0 | 0xFF,
			1 => self.reg.nrx1 | 0xFF,
			2 => self.reg.nrx2 | 0x00,
			3 => self.reg.nrx3 | 0x00,
			4 => self.reg.nrx4 | 0xBF,
			_ => unreachable!("NoiseChannel::get_reg()"),
		}
	}

	pub fn set_reg(&mut self, register: u8, value: u8) {
		match register {
			0 => self.reg.nrx0 = value,
			1 => {
				self.reg.nrx1 = value;
				let six_bits = value & 0x3F;
				self.len_timer = 64 - six_bits;
			}
			2 => {
				self.reg.nrx2 = value;
				if !self.dac_status() {
					self.active = false;
				}
			},
			3 => self.reg.nrx3 = value,
			4 => {
				self.reg.nrx4 = value;
				if (self.reg.nrx4 >> 7 & 0x01) == 1 {
					self.trigger();
				}
			}
			_ => unreachable!("NoiseChannel::get_reg()"),
		};
	}

	// Returns the status of the DAC
	pub fn dac_status(&self) -> bool {
		if self.reg.nrx2 & 0xF8 != 0 {
			true
		} else {
			false
		}
	}

	pub fn clear(&mut self) {
		self.reg.clear();
		self.active = false;
	}

	fn trigger(&mut self) {
		if self.len_timer == 0 {
			self.len_timer = 64;
		}
		if self.dac_status() {
			self.active = true;
		}
	}
	
	fn length_enable(&self) -> bool {
		if (self.reg.nrx4 >> 6) & 0x01 == 1 {
			true
		} else {
			false
		}
	}
}
