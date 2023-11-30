pub mod romonly;
pub mod mbc1;

use std::fs::File;
use std::io::Read;

use romonly::RomOnly;
use mbc1::MBC1;

pub fn load(path: &str) -> Box<dyn Cartridge> {
    let mut rom = File::open(path).expect("Unable to open file {path}");
    let mut data_buffer = Vec::new();
    rom.read_to_end(&mut data_buffer).unwrap();

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
    println!("Cartridge type: {:02X}", cartridge_type);
    println!("Number of RAM banks: {}", ram_banks);
    let cartridge: Box<dyn Cartridge> = match cartridge_type {
        0x00 => Box::new(RomOnly::new(&data_buffer)),
        0x01 => Box::new(MBC1::new(&data_buffer, ram_banks)),
        _ => unreachable!("Cartridge::load()"),
    };
    cartridge
}

pub trait Cartridge {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
