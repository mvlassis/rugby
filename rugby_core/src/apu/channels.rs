use serde::{Serialize, Deserialize};

const WAVE_FORMS: [[u8; 8]; 4] = [
	[0, 0, 0, 0, 0, 0, 0, 1],
	[1, 0, 0, 0, 0, 0, 0, 1],
	[1, 0, 0, 0, 0, 1, 1, 1],
	[0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
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

#[derive(Clone, Copy, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum ChannelType {
	Pulse1,
	Pulse2,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct PulseChannel {
	pub reg: SoundRegisters,
	pub channel: ChannelType,

	volume: u8,
	frequency_timer: u16,
	duty_step: u8,
	len_timer: u8,
	volume_timer: u8,
	sweep_timer: u8,
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
		let frequency_timer = (0xBF as u16 & 0b111) << 8 | (0xFF as u16);
		PulseChannel {
			reg,
			channel,
			volume: 0,
			frequency_timer,
			duty_step: 0,
			len_timer: 0,
			volume_timer: 0,
			sweep_timer: 0,
			active: false,
		}
	}

	// Called every T-Cycle
	pub fn duty_cycle(&mut self) {
		self.frequency_timer -= 1;

		if self.frequency_timer == 0 {
			self.frequency_timer = (2048 - self.get_frequency()) * 4;
			self.duty_step = (self.duty_step + 1) % 8; // Move to the next sample
		}
	}
	
	pub fn tick(&mut self, div_apu: u8) {
		// Tick the length counter
		if div_apu % 2 == 0 && self.length_enable() && self.len_timer > 0 {
			self.len_timer -= 1;
			if self.len_timer == 0 {
				self.active = false;
			}
		}

		// Tick the frequency sweep (only on channel 1)
		if div_apu % 4 == 0 && self.channel == ChannelType::Pulse1 {
			let sweep_pace = (self.reg.nrx0 & 0x70) >> 4; // Bits 4-7
			if sweep_pace != 0 {
				if self.sweep_timer > 0 {
					self.sweep_timer -= 1;
				}
				if self.sweep_timer == 0 {
					self.sweep_timer = sweep_pace;
					let frequency = self.get_frequency();
					let sweep_step = self.reg.nrx0 & 0b111; // Bits 0-2
					let new_frequency = if (self.reg.nrx0 >> 3) & 0x01 == 0 {
						frequency + (frequency / 2u16.pow(sweep_step as u32))
					} else {
						frequency - (frequency / 2u16.pow(sweep_step as u32))
					};

					if (self.reg.nrx0 >> 3) & 0x01 == 0 && new_frequency > 0x7FF {
						self.active = false;
					}

					self.reg.nrx3 = new_frequency as u8;
					self.reg.nrx4 |= ((new_frequency & 0x700) >> 8) as u8;
				}
			}
		}

		// Tick the volume envelope
		if div_apu % 8 == 0 && self.reg.nrx2 & 0b111 != 0 {
			if self.volume_timer > 0 {
				self.volume_timer -= 1;
			}
			if self.volume_timer == 0 {
				self.volume_timer = self.reg.nrx2 & 0b111; // Bits 0-2

				if self.reg.nrx2 & 0x08 == 0 {
                    if self.volume > 0 {
                        self.volume -= 1;
                    }
                } else {
                    if self.volume < 0xF {
                        self.volume += 1;
                    }
                }
			}
		}
	}

	// Returns a sample from this channel
	pub fn get_sample(&self) -> f32 {
		let wave_form = ((self.reg.nrx1 >> 6) & 0b11) as usize;
		let sample = WAVE_FORMS[wave_form][self.duty_step as usize] * self.volume;

		if self.dac_status() {
			(sample as f32 / 7.5) - 1.0
		} else {
			0.0
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
		self.duty_step = 0;
		self.active = false;
	}

	// Returns the status of the DAC
	pub fn dac_status(&self) -> bool {
		if self.reg.nrx2 & 0xF8 != 0 {
			true
		} else {
			false
		}
	}
	
	fn trigger(&mut self) {
		if self.len_timer == 0 {
			self.len_timer = 64;
		}
		self.volume_timer = self.reg.nrx2 & 0b111;
		self.volume = (self.reg.nrx2 & 0xF0) >> 4;
		self.frequency_timer = (2048 - self.get_frequency()) * 4;
		self.sweep_timer = 0;
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

	fn get_frequency(&self) -> u16 {
		(self.reg.nrx4 as u16 & 0b111) << 8 | (self.reg.nrx3 as u16)
	}
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct WaveChannel {
	pub reg: SoundRegisters,
	pub wave_pattern: [u8; 16],
	wave_index: u8,
	frequency_timer: u16,
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
		let frequency_timer = (0xBF as u16 & 0b111) << 8 | (0xFF as u16);
		WaveChannel {
			reg,
			wave_pattern: [0; 16],
			wave_index: 0,
			frequency_timer,
			len_timer: 0,
			active: false,
		}
	}

	// Called every T-Cycle
	pub fn duty_cycle(&mut self) {
		self.frequency_timer -= 1;

		if self.frequency_timer == 0 {
			self.frequency_timer = (2048 - self.get_frequency()) * 2;
			self.wave_index = (self.wave_index + 1) % 32; // Move to the next sample
		}
	}

	// Returns a sample from this channel
	pub fn get_sample(&self) -> f32 {
		let raw_sample = if self.wave_index % 2 == 0 {
            (self.wave_pattern[(self.wave_index / 2) as usize] & 0xF0) >> 4
        } else {
            self.wave_pattern[(self.wave_index / 2) as usize] & 0x0F
        };

        let sample = match (self.reg.nrx2 & 0x60) >> 5 {
            0b00 => 0,
            0b01 => raw_sample,
            0b10 => raw_sample >> 1,
            0b11 => raw_sample >> 2,
            _ => unreachable!("WaveChannel::get_sample()"),
        };

		if self.dac_status() {
			(sample as f32 / 7.5) - 1.0
		} else {
			0.0
		}
	}
	
	pub fn tick(&mut self, div_apu: u8) {
		// Tick the length counter
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
		self.frequency_timer = (2048 - self.get_frequency()) * 2;
		self.wave_index = 1;
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

	fn get_frequency(&self) -> u16 {
		(self.reg.nrx4 as u16 & 0b111) << 8 | (self.reg.nrx3 as u16)
	}
}


#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct NoiseChannel {
	pub reg: SoundRegisters,
	lsfr: u16,
	len_timer: u8,
	frequency_timer: u16,
	volume: u8,
	volume_timer: u8,
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
		let base_divisor = match reg.nrx4 & 0b111 {
			0 => 8 as u16,
			_ => (reg.nrx4 as u16 & 0b111) * 16
		};
		let shift_amount = (reg.nrx3 & 0xF0) >> 4;
		let frequency_timer = base_divisor << shift_amount;
		NoiseChannel {
			reg,
			lsfr: 0,
			len_timer: 0,
			frequency_timer,
			volume: 0,
			volume_timer: 0,
			active: false,
		}
	}

	// Called every T-Cycle
	pub fn duty_cycle(&mut self) {
		self.frequency_timer = self.frequency_timer.wrapping_sub(1);

		if self.frequency_timer == 0 {
			// Update the frequency timer
			let base_divisor = self.get_divisor() as u16;
			let shift_amount = (self.reg.nrx3 & 0xF0) >> 4;
			self.frequency_timer = base_divisor << shift_amount;

			// Update LSFR
			let xor_result = (self.lsfr & 0b01) ^ ((self.lsfr & 0b10) >> 1);
			self.lsfr = (self.lsfr >> 1) & !(1 << 14);
			self.lsfr |= xor_result << 14;

			if self.reg.nrx3 & 0x08 != 0 {
				self.lsfr &= !(1 << 6);
				self.lsfr |= xor_result << 6;
			}
		}
	}
	
	pub fn tick(&mut self, div_apu: u8) {
		// Tick the length counter
		if div_apu % 2 == 0 && self.length_enable() && self.len_timer > 0 {
			self.len_timer -= 1;
			if self.len_timer == 0 {
				self.active = false;
			}
		}

		// Tick the volume envelope
		if div_apu % 8 == 0 && self.reg.nrx2 & 0b111 != 0 {
			if self.volume_timer > 0 {
				self.volume_timer -= 1;
			}
			if self.volume_timer == 0 {
				self.volume_timer = self.reg.nrx2 & 0b111; // Bits 0-2

				if self.reg.nrx2 & 0x08 == 0 {
                    if self.volume > 0 {
                        self.volume -= 1;
                    }
                } else {
                    if self.volume < 0xF {
                        self.volume += 1;
                    }
                }
			}
		}
	}

	// Returns a sample from this channel
	pub fn get_sample(&self) -> f32 {
		let sample = (!self.lsfr as u8 & 0x01) * self.volume;
		if self.dac_status() {
			(sample as f32 / 7.5) - 1.0
		} else {
			0.0
		}
	}
	
	pub fn get_reg(&self, register: u8) -> u8 {
		match register {
			0 => 0xFF,
			1 => self.reg.nrx1 | 0xFF,
			2 => self.reg.nrx2 | 0x00,
			3 => self.reg.nrx3 | 0x00,
			4 => self.reg.nrx4 | 0xBF,
			_ => unreachable!("NoiseChannel::get_reg()"),
		}
	}
	
	pub fn set_reg(&mut self, register: u8, value: u8) {
		match register {
			0 => (),
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
			_ => unreachable!("NoiseChannel::set_reg()"),
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
		self.lsfr = u16::MAX;
		self.volume_timer = self.reg.nrx2 & 0b111;
		self.volume = (self.reg.nrx2 & 0xF0) >> 4;

		// Update the frequency timer
		let base_divisor = self.get_divisor();
		let shift_amount = (self.reg.nrx3 & 0xF0) >> 4;
		self.frequency_timer = base_divisor << shift_amount;
		
		if self.dac_status() {
			self.active = true;
		}
	}

	// Get the clock divisor
	fn get_divisor(&self) -> u16 {
		match self.reg.nrx3 & 0b111 {
			0 => 8,
			_ => (self.reg.nrx3 as u16 & 0b111) * 16
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
