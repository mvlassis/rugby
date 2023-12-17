pub mod romonly;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod mbc5;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use romonly::RomOnly;
use mbc1::MBC1;
use mbc2::MBC2;
use mbc3::MBC3;
use mbc5::MBC5;

pub const BANK_SIZE: usize = 16384;

pub fn load(path_buf: PathBuf) -> Box<dyn Cartridge> {
    let mut save_path = path_buf.clone();
    if let Some(file_stem) = save_path.file_stem() {
        save_path = save_path.with_file_name(file_stem).with_extension("sav");
    }
    
    let mut rom = File::open(path_buf).expect("Unable to open file {path}");
    let mut data_buffer = Vec::new();
    rom.read_to_end(&mut data_buffer).unwrap();

	let _rom_size = BANK_SIZE * (2 << data_buffer[0x0148]);
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
    // println!("Cartridge type: {:02X}", cartridge_type);
    // println!("ROM size: {}", rom_size);
    // println!("Number of RAM banks: {}", ram_banks);
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
    cartridge
}

pub trait Cartridge {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
	fn save(&mut self);
	fn update_clock(&mut self);
}
