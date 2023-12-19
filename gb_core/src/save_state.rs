use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;

use crate::input::Input;
use crate::timer::Timer;
use crate::ppu::{Object, Mode};
use crate::apu::channels::PulseChannel;
use crate::apu::channels::WaveChannel;
use crate::apu::channels::NoiseChannel;


#[derive(Serialize, Deserialize)]
pub struct EmulatorState {
	pub cpu_state: CPUState,
	pub bus_state: BusState,
	pub cartridge_json: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusState {
	pub mmu_state: MMUState,
	pub ppu_state: PPUState,
	pub apu_state: APUState,
}

// Missing the lookup table
#[derive(Serialize, Deserialize)]
pub struct CPUState {
	pub cpu_registers: [u8; 10],
	pub pc: u16,
	pub mcycles: u8,
	pub ime: u8,
	pub ime_scheduled: bool,
	pub halt_mode: bool,
	pub rtc_oscillator: u64,
}

// Missing the cartridge
#[derive(Serialize, Deserialize)]
pub struct MMUState {
	#[serde(with = "BigArray")]
	pub wram:         [u8; 8192],
	#[serde(with = "BigArray")]
	pub io_registers: [u8; 128],
	#[serde(with = "BigArray")]
	pub hram:         [u8; 127],
	pub ie_register: u8,

	pub timer: Timer,
	pub input: Input,
	pub prev_p1: u8,
	pub joypad_interrupt: bool,
	#[serde(with = "BigArray")]
	pub serial_buffer: [u8; 100],
	pub serial_file_path: String,
}

// Missing screen
#[derive(Serialize, Deserialize)]
pub struct PPUState {
    pub lcdc: u8,
    pub ly: u8,
    pub lyc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub wy: u8,
    pub wx: u8,
    pub window_line_counter: u8,
    pub window_in_frame: bool,
    pub window_in_line: bool,
    pub object_buffer: Vec<Object>,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
	#[serde(with = "BigArray")]
    pub vram: [u8; 8192],
	#[serde(with = "BigArray")]
    pub oam: [u8; 160],
	// TODO Maybe add screen buffer?
    // pub screen_buffer: [[Color; GB_WIDTH]; GB_HEIGHT],
    pub mode: Mode,
    pub current_clock: u16,
    pub vblank_interrupt: bool,
    pub stat_interrupt: bool,
    pub prev_interrupt_line: u8,
    pub frame_ready: bool,
	pub ppu_disabled: bool,
}

// Missing the callback function
#[derive(Serialize, Deserialize)]
pub struct APUState {
	pub buffer: Vec<f32>,
	pub buffer_position: usize,
	pub is_buffer_full: bool,
	pub channel1: PulseChannel,
	pub channel2: PulseChannel,
	pub channel3: WaveChannel,
	pub channel4: NoiseChannel,
	pub mute_channels: [bool; 4],
	pub nr50: u8, 
	pub nr51: u8, 
	pub nr52: u8, 
	pub div_apu: u8,
	pub prev_div_apu: u8,
	pub capacitor: f32,
	pub internal_cycles: u16,
	pub is_mute: bool,
}
