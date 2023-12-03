use super::Cartridge;
use super::BANK_SIZE;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;


// Get bit at a specific position
fn get_bit(value: u8, bit_position: u8) -> u8 {
	let bit = (value >> bit_position) & 0x1;
	bit as u8
}

// Set bit at a specific position to a specific bit value
fn set_bit(value: usize, bit_position: u8, bit_value: u8) -> usize {
	let new_value = match bit_value {
		0 => value & !(1 << bit_position),
		1 => value | 1 << bit_position,
		_ => unreachable!("MBC5::set_bit()"),
	};
	new_value
}

pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    
    ram_enable: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,

	rom_bit_mask: usize,

    save_path: Option<PathBuf>
}

impl MBC5 {
    pub fn new(data_buffer: &Vec<u8>, ram_banks: usize, save_path: Option<PathBuf>) -> Self {
        let mut rom: Vec<u8> = Vec::new();
        rom.extend_from_slice(data_buffer);
        let mut ram: Vec<u8> = Vec::new();
		let rom_banks = rom.len() / BANK_SIZE;
		let rom_bits = (rom_banks as f64).log2().ceil() as usize;
		let rom_bit_mask = (1 << rom_bits) - 1;

		match save_path.clone() {
			Some(path) => {
				if path.exists() {
					let result = File::open(path);
					match result {
						Ok(mut file) => {
							let _ = file.read_to_end(&mut ram);
						}
						Err(_) => {
							
						}
					}
				} else {
					ram = vec![0; ram_banks * BANK_SIZE];
				}
			},
			None => {
				ram = vec![0; ram_banks * BANK_SIZE];
			},
		}
		
        MBC5 {
            rom,
            ram,
            
            ram_enable: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
			rom_bit_mask,
            save_path
        }
    }
	
}


impl Cartridge for MBC5 {
    
    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x3FFF => {
                self.rom[address]
            }
            0x4000..=0x7FFF => {
                let address = address - 0x4000;
                self.rom[address + (self.rom_bank_number & self.rom_bit_mask) * BANK_SIZE]
            }
            0xA000..=0xBFFF => {
                let address = address - 0xA000;
                if self.ram_enable {
                    self.ram[address + (self.ram_bank_number) * BANK_SIZE]
                } else {
					0xFF
				}
            }
            _ => unreachable!("MBC5::read() at address: {:04X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x1FFF => {
                let low_nibble = value & 0x0F;
                if low_nibble == 0x0A {
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
            }
            0x2000..=0x2FFF => {
				self.rom_bank_number &= !0xFF;
                self.rom_bank_number |= value as usize;
            }
			0x3000..=0x3FFF => {
				let one_bit = value & 0x01;
				self.rom_bank_number = set_bit(self.rom_bank_number, 8, one_bit);
			}
            0x4000..=0x5FFF => {
                let four_bits = value & 0x0F;
                self.ram_bank_number = four_bits as usize;
            }
            0x6000..=0x9FFF => {
                ()
				
            }
            0xA000..=0xBFFF => {
				let address = address - 0xA000;
                if self.ram_enable {
                    self.ram[address + (self.ram_bank_number) * BANK_SIZE] = value;
                } else {
                    ()
                }

            }
            _ => unreachable!("MBC5::write() at address: {:04X}", address),
        }
    }

	fn save(&mut self) {
        if let Some(save_path) = &self.save_path {
			let result = File::create(&save_path);
            match result {
                Ok(mut file) => {
                    let _ = file.write_all(&self.ram);
                },
                Err(e) => {
                    eprintln!("MBC5: An error occured: {}", e);
                }
            }
        }
    }

	fn update_clock(&mut self) {
		
	}
}
