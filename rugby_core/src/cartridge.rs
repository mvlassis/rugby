pub mod romonly;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod mbc5;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::gb_mode::GBMode;
use romonly::RomOnly;
use mbc1::MBC1;
use mbc2::MBC2;
use mbc3::MBC3;
use mbc5::MBC5;

pub const ROM_BANK_SIZE: usize = 16384;
pub const RAM_BANK_SIZE: usize = 8192;

// Returns a cartridge, and whether it is for Gameboy or Gameboy Color
pub fn load(path_buf: Option<PathBuf>) -> (Box<dyn Cartridge>, GBMode) {
	// If no path is given, create a dummy ROM
	if path_buf.is_none() {
		let data_buffer = vec![0u8; 32768];
		let cartridge = Box::new(RomOnly::new(&data_buffer));
		return (cartridge, GBMode::DMG);
	}
	
    let mut save_path = path_buf.clone().unwrap();
    if let Some(file_stem) = save_path.file_stem() {
        save_path = save_path.with_file_name(file_stem).with_extension("sav");
    }
    
    let mut rom = File::open(path_buf.unwrap()).expect("Unable to open file {path}");
    let mut data_buffer = Vec::new();
    rom.read_to_end(&mut data_buffer).unwrap();

	let _rom_size = ROM_BANK_SIZE * (2 << data_buffer[0x0148]);
    let cartridge_type = data_buffer[0x0147];
    let ram_banks = match data_buffer[0x0149] {
        0x00 => 0,
        0x01 => 0,
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => unreachable!("Cartridge::load(), ram_banks"),
    };

	let gb_mode = match data_buffer[0x0143] {
		0x80 | 0xC0 => GBMode::CGB,
		_ => GBMode::DMG,
	};
	
    // println!("Cartridge type: {:02X}", cartridge_type);
    // println!("ROM size: {}", rom_size);
    // println!("Number of RAM banks: {}", ram_banks);
	// println!("GBMode: {:?}", gb_mode);
    let cartridge: Box<dyn Cartridge> = match cartridge_type {
        0x00 => Box::new(RomOnly::new(&data_buffer)),
        0x01 | 0x02 => Box::new(MBC1::new(&data_buffer, ram_banks, None)),
        0x03 => Box::new(MBC1::new(&data_buffer, ram_banks, Some(save_path))),
		0x05 => Box::new(MBC2::new(&data_buffer, None)),
		0x06 => Box::new(MBC2::new(&data_buffer, Some(save_path))),
		0x0F | 0x10 => Box::new(MBC3::new(&data_buffer, ram_banks, Some(save_path), true)),
		0x11 | 0x12 => Box::new(MBC3::new(&data_buffer, ram_banks, None, false)),
		0x13 => Box::new(MBC3::new(&data_buffer, ram_banks, Some(save_path), false)),
		0x19 | 0x1A => Box::new(MBC5::new(&data_buffer, ram_banks, None)),
		0x1B => Box::new(MBC5::new(&data_buffer, ram_banks, Some(save_path))),
		0x1C | 0x1D => Box::new(MBC5::new(&data_buffer, ram_banks, None)),
		0x1E => Box::new(MBC5::new(&data_buffer, ram_banks, Some(save_path))),
        _ => unreachable!("Cartridge::load()"),
    };
    (cartridge, gb_mode)
}

pub trait Cartridge {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
	fn save(&mut self);
	fn update_clock(&mut self);

	fn create_state(&self) -> String;
	fn load_state(&mut self, json_string: &str);
}
