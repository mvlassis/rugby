use rodio::{OutputStream, OutputStreamHandle, buffer::SamplesBuffer, Sink};

mod channels;

use crate::apu::channels::ChannelType;
use crate::apu::channels::PulseChannel;
use crate::apu::channels::WaveChannel;
use crate::apu::channels::NoiseChannel;

const AUDIO_BUFFER_SIZE: usize = 1200;

pub struct APU {
	pub sink: Sink,
	_stream: (OutputStream, OutputStreamHandle),
	
	channel1: PulseChannel,
	channel2: PulseChannel,
	channel3: WaveChannel,
	channel4: NoiseChannel,
	
	pub nr50: u8, // Master volume
	pub nr51: u8, // Sound panning
	pub nr52: u8, // Audio master control
	div_apu: u8,
	prev_div_apu: u8,
	audio_buffer: Vec<f32>,
	capacitor: f32,
	internal_cycles: u16, // Tracks the current cycle
}

impl APU {
	pub fn new() -> Self {
		let (stream, stream_handle) = OutputStream::try_default().unwrap();
		let sink = Sink::try_new(&stream_handle).unwrap();
		
		let channel1 = PulseChannel::new(ChannelType::Pulse1);
		let channel2 = PulseChannel::new(ChannelType::Pulse2);
		let channel3 = WaveChannel::new();
		let channel4 = NoiseChannel::new();
		APU {
			sink,
			_stream: (stream, stream_handle),
			channel1,
			channel2,
			channel3,
			channel4,
			
			nr50: 0x77,
			nr51: 0xF3,
			nr52: 0xF1,
			div_apu: 0,
			prev_div_apu: 0,
			audio_buffer: Vec::with_capacity(AUDIO_BUFFER_SIZE),
			capacitor: 0.0,
			internal_cycles: 0,
		}
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

	fn mix(&mut self) {
		let channel1_sample = if self.channel1.active {
			self.channel1.get_sample()
		} else {
			0.0
		};
		let channel2_sample = if self.channel2.active {
			self.channel2.get_sample()
		} else {
			0.0
		};
		let channel3_sample = if self.channel3.active {
			self.channel3.get_sample()
		} else {
			0.0
		};
		let channel4_sample = if self.channel4.active {
			self.channel4.get_sample()
		} else {
			0.0
		};

		let mut left_mix_sample = channel1_sample + channel2_sample
			+ channel3_sample + channel4_sample;
		let mut right_mix_sample = channel1_sample + channel2_sample
			+ channel3_sample + channel4_sample;

		left_mix_sample /= 4.0;
		right_mix_sample /= 4.0;
		
		let ls = self.high_pass(left_mix_sample);
		let rs = self.high_pass(right_mix_sample);

		self.audio_buffer.extend([ls, rs]);
		if self.audio_buffer.len() >= AUDIO_BUFFER_SIZE {
			while self.sink.len() > 2 {}
            self.sink.append(SamplesBuffer::new(2, 44100, self.audio_buffer.clone()));
			self.audio_buffer.clear();
		}
	}

	// Simulates a high pass filter
	fn high_pass(&mut self, in_sample: f32) -> f32 {
        let out = in_sample - self.capacitor;
        self.capacitor = in_sample - out * 0.99601;
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
}

