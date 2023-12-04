mod channels;

use crate::apu::channels::ChannelType;
use crate::apu::channels::PulseChannel;
use crate::apu::channels::WaveChannel;
use crate::apu::channels::NoiseChannel;

pub struct APU {
	channel1: PulseChannel,
	channel2: PulseChannel,
	channel3: WaveChannel,
	channel4: NoiseChannel,
	
	pub nr50: u8, // Master volume
	pub nr51: u8, // Sound panning
	pub nr52: u8, // Audio master control
	div_apu: u8,
	prev_div_apu: u8,
}

impl APU {
	pub fn new() -> Self {
		let channel1 = PulseChannel::new(ChannelType::Pulse1);
		let channel2 = PulseChannel::new(ChannelType::Pulse2);
		let channel3 = WaveChannel::new();
		let channel4 = NoiseChannel::new();
		APU {
			channel1,
			channel2,
			channel3,
			channel4,
			
			nr50: 0x77,
			nr51: 0xF3,
			nr52: 0xF1,
			div_apu: 0,
			prev_div_apu: 0,
		}
	}

	pub fn tick(&mut self, div: u8) {
		// Increment the DIV APU when detecting a falling edge
		if (self.prev_div_apu >> 4) & 0x01 == 1 && (div >> 4) & 0x01 == 0 {
			self.div_apu = self.div_apu.wrapping_add(1);
			self.channel1.tick(self.div_apu);
			self.channel2.tick(self.div_apu);
			self.channel3.tick(self.div_apu);
			self.channel4.tick(self.div_apu);
		}
		self.prev_div_apu = div;
	}

	// Get 8-bit value from memory at a specific address
	pub fn get_byte(&self, address: u16) -> u8 {
		match address {
			0xFF10..=0xFF3F => {
				match address {
					0xFF10 => self.channel1.get_reg(0),
					0xFF11 => self.channel1.get_reg(1),
					0xFF12 => self.channel1.get_reg(2),
					0xFF13 => self.channel1.get_reg(3),
					0xFF14 => self.channel1.get_reg(4),
					0xFF15 => self.channel2.get_reg(0),
					0xFF16 => self.channel2.get_reg(1),
					0xFF17 => self.channel2.get_reg(2),
					0xFF18 => self.channel2.get_reg(3),
					0xFF19 => self.channel2.get_reg(4),
					0xFF1A => self.channel3.get_reg(0),
					0xFF1B => self.channel3.get_reg(1),
					0xFF1C => self.channel3.get_reg(2),
					0xFF1D => self.channel3.get_reg(3),
					0xFF1E => self.channel3.get_reg(4),
					0xFF1F => self.channel4.get_reg(0),
					0xFF20 => self.channel4.get_reg(1),
					0xFF21 => self.channel4.get_reg(2),
					0xFF22 => self.channel4.get_reg(3),
					0xFF23 => self.channel4.get_reg(4),
					0xFF24 => self.nr50,
					0xFF25 => self.nr51,
					0xFF26 => {
						let mut value = 0x70;
						if self.is_enabled() {
							value |= 0x80;
						}
						if self.channel1.active {
							value |= 0x1
						}
						if self.channel2.active {
							value |= 0x2
						}
						if self.channel3.active {
							value |= 0x4
						}
						if self.channel4.active {
							value |= 0x8
						}
						// println!("NR52: {:08b}", value);
						value
					}
					0xFF27..=0xFF2F => 0xFF,
					0xFF30..=0xFF3F => self.channel3.wave_pattern[address as usize
					- 0xFF30],
					_ => unreachable!("APU::get_byte at address: {:04X}", address),
				}
			},
			_ => unreachable!("APU::get_byte(): Out of memory at address: {:04X}", address),
		}
	}

	// Set an 8-bit value at a specific address in memory
	pub fn set_byte(&mut self, address: u16, value: u8) {
		if address == 0xFF26 {
			self.nr52 = value;
			if !self.is_enabled() {
				self.channel1.clear();
				self.channel2.clear();
				self.channel3.clear();
				self.channel4.clear();
				self.nr50 = 0;
				self.nr51 = 0;
			}
		}
		if address != 0xFF26 && !self.is_enabled() {
			return;
		}
		match address {
			0xFF10..=0xFF3F => {
				match address {
					0xFF10 => self.channel1.set_reg(0, value),
					0xFF11 => self.channel1.set_reg(1, value),
					0xFF12 => self.channel1.set_reg(2, value),
					0xFF13 => self.channel1.set_reg(3, value),
					0xFF14 => self.channel1.set_reg(4, value),
					0xFF15 => (),
					0xFF16 => self.channel2.set_reg(1, value),
					0xFF17 => self.channel2.set_reg(2, value),
					0xFF18 => self.channel2.set_reg(3, value),
					0xFF19 => self.channel2.set_reg(4, value),
					0xFF1A => self.channel3.set_reg(0, value),
					0xFF1B => self.channel3.set_reg(1, value),
					0xFF1C => self.channel3.set_reg(2, value),
					0xFF1D => self.channel3.set_reg(3, value),
					0xFF1E => self.channel3.set_reg(4, value),
					0xFF1F => (),
					0xFF20 => self.channel4.set_reg(1, value),
					0xFF21 => self.channel4.set_reg(2, value),
					0xFF22 => self.channel4.set_reg(3, value),
					0xFF23 => self.channel4.set_reg(4, value),
					0xFF24 => self.nr50 = value,
					0xFF25 => self.nr51 = value,
					0xFF26 => self.nr52 = value,
					0xFF27..=0xFF2F => (),
					0xFF30..=0xFF3F => self.channel3.wave_pattern[address as usize
					- 0xFF30] = value,
					_ => unreachable!("APU::set_byte at address: {:04X}", address),
				}
			},
			_ => unreachable!("APU::set_byte(): Out of memory at address: {:04X}", address),
		}
	}

	// True if APU is enabled, false if it is disabled
	fn is_enabled(&self) -> bool {
		if (self.nr52 >> 7) & 0x01 == 1 {
			true
		} else {
			false
		}
	}
}

