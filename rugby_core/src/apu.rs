pub mod channels;

use crate::apu::channels::ChannelType;
use crate::apu::channels::PulseChannel;
use crate::apu::channels::WaveChannel;
use crate::apu::channels::NoiseChannel;
use crate::save_state::APUState;

pub const AUDIO_BUFFER_SIZE: usize = 1024;

pub struct APU {
	callback: Box<dyn Fn(&[f32])>,
	pub buffer: Vec<f32>,
	pub buffer_position: usize,
	pub is_buffer_full: bool,
	
	channel1: PulseChannel,
	channel2: PulseChannel,
	channel3: WaveChannel,
	channel4: NoiseChannel,
	mute_channels: [bool; 4],

	pub nr50: u8, // Master volume
	pub nr51: u8, // Sound panning
	pub nr52: u8, // Audio master control
	div_apu: u8,
	prev_div_apu: u8,
	capacitor: f32,
	internal_cycles: u16, // Tracks the current cycle

	is_mute: bool,
}

impl APU {
	pub fn new(callback: Box<dyn Fn(&[f32])>) -> Self {
		
		let channel1 = PulseChannel::new(ChannelType::Pulse1);
		let channel2 = PulseChannel::new(ChannelType::Pulse2);
		let channel3 = WaveChannel::new();
		let channel4 = NoiseChannel::new();
		APU {
			callback,
			buffer: vec![0.0; AUDIO_BUFFER_SIZE],
			buffer_position: 0,
			is_buffer_full: false,
			
			channel1,
			channel2,
			channel3,
			channel4,
			mute_channels: [false; 4],
			
			nr50: 0x77,
			nr51: 0xF3,
			nr52: 0xF1,
			div_apu: 0,
			prev_div_apu: 0,
			capacitor: 0.0,
			internal_cycles: 0,
			is_mute: false,
		}
	}

	// Resets the APU (using the same callback function)
	pub fn reset(&mut self) {
		self.channel1 = PulseChannel::new(ChannelType::Pulse1);
		self.channel2 = PulseChannel::new(ChannelType::Pulse2);
		self.channel3 = WaveChannel::new();
		self.channel4 = NoiseChannel::new();

		self.buffer = vec![0.0; AUDIO_BUFFER_SIZE];
		self.buffer_position = 0;
		self.is_buffer_full = false;
		
		self.nr50 = 0x77;
		self.nr51 = 0xF3;
		self.nr52 = 0xF1;
		self.div_apu = 0;
		self.prev_div_apu = 0;
		self.capacitor = 0.0;
		self.internal_cycles = 0;
	}

	pub fn toggle_mute(&mut self) {
		self.is_mute = !self.is_mute;
	}

	pub fn toggle_channel(&mut self, i: usize) {
		self.mute_channels[i] = !self.mute_channels[i];
	}
	
	pub fn tick(&mut self, div: u8) {
		self.channel1.duty_cycle();
		self.channel2.duty_cycle();
		self.channel3.duty_cycle();
		self.channel4.duty_cycle();

		// Increment the DIV APU when detecting a falling edge
		if self.is_enabled() && (self.prev_div_apu >> 4) & 0x01 == 1 && (div >> 4) & 0x01 == 0 {
			self.div_apu = self.div_apu.wrapping_add(1);
			self.channel1.tick(self.div_apu);
			self.channel2.tick(self.div_apu);
			self.channel3.tick(self.div_apu);
			self.channel4.tick(self.div_apu);
		}
		self.prev_div_apu = div;

		// Magic number = (apu_freq / 44.1kHz), apu_freq = 2^222
		if self.internal_cycles >= 95 {
			self.internal_cycles = 0;
			self.mix();
		}
		self.internal_cycles += 1;
	}

	// Fills the audio buffer with a new sample
	fn mix(&mut self) {
		let channel1_sample = if self.channel1.active && !self.mute_channels[0] {
			self.channel1.get_sample()
		} else {
			0.0
		};
		let channel2_sample = if self.channel2.active && !self.mute_channels[1] {
			self.channel2.get_sample()
		} else {
			0.0
		};
		let channel3_sample = if self.channel3.active && !self.mute_channels[2] {
			self.channel3.get_sample()
		} else {
			0.0
		};
		let channel4_sample = if self.channel4.active && !self.mute_channels[3] {
			self.channel4.get_sample()
		} else {
			0.0
		};

		let mut left_mix_sample = match self.is_mute {
			true => 0.0,
			false => channel1_sample + channel2_sample
			+ channel3_sample + channel4_sample,
		};
		let mut right_mix_sample = match self.is_mute {
			true => 0.0,
			false => channel1_sample + channel2_sample
			+ channel3_sample + channel4_sample
		} ;

		left_mix_sample /= 4.0;
		right_mix_sample /= 4.0;
		
		let ls = self.high_pass(left_mix_sample);
		let rs = self.high_pass(right_mix_sample);

		self.buffer[self.buffer_position] = ls;
		self.buffer[self.buffer_position + 1] = rs;
		self.buffer_position += 2;
		
		if self.buffer_position >= AUDIO_BUFFER_SIZE {
			(self.callback)(self.buffer.as_ref());
			self.buffer_position = 0;
		}
	}

	// Simulates a high pass filter
	fn high_pass(&mut self, in_sample: f32) -> f32 {
        let out = in_sample - self.capacitor;
        self.capacitor = in_sample - out * 0.996;
        out
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

	// Creates an APUState from the APU
	pub fn create_state(&self) -> APUState {
		APUState {
			buffer: self.buffer.clone(),
			buffer_position: self.buffer_position,
			is_buffer_full: self.is_buffer_full,
			channel1: self.channel1.clone(),
			channel2: self.channel2.clone(),
			channel3: self.channel3.clone(),
			channel4: self.channel4.clone(),
			mute_channels: self.mute_channels.clone(),
			nr50: self.nr50,
			nr51: self.nr51,
			nr52: self.nr52,
			div_apu: self.div_apu,
			prev_div_apu: self.prev_div_apu,
			capacitor: self.capacitor,
			internal_cycles: self.internal_cycles,
			is_mute: self.is_mute,
		}
	}

	// Loads an APUState to the APU
	pub fn load_state(&mut self, apu_state: APUState) {
		self.buffer = apu_state.buffer.clone();
		self.buffer_position = apu_state.buffer_position;
		self.is_buffer_full = apu_state.is_buffer_full;
		self.channel1 = apu_state.channel1.clone();
		self.channel2 = apu_state.channel2.clone();
		self.channel3 = apu_state.channel3.clone();
		self.channel4 = apu_state.channel4.clone();
		self.mute_channels = apu_state.mute_channels.clone();
		self.nr50 = apu_state.nr50;
		self.nr51 = apu_state.nr51;
		self.nr52 = apu_state.nr52;
		self.div_apu = apu_state.div_apu;
		self.prev_div_apu = apu_state.prev_div_apu;
		self.capacitor = apu_state.capacitor;
		self.internal_cycles = apu_state.internal_cycles;
		self.is_mute = apu_state.is_mute;
	}
}

