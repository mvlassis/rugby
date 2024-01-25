use super::Cartridge;
use super::ROM_BANK_SIZE;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;

#[derive(Serialize, Deserialize)]
pub struct MBC2 {
	#[serde(skip)]
    rom: Vec<u8>,
	#[serde(with = "BigArray")]
    ram: [u8; 512],
    
    ram_enable: bool,
    rom_bank_number: usize,

    save_path: Option<PathBuf>
}

impl MBC2 {
    pub fn new(data_buffer: &Vec<u8>, save_path: Option<PathBuf>) -> Self {
        let mut rom: Vec<u8> = Vec::new();
        rom.extend_from_slice(data_buffer);
		let mut ram = [0; 512];

		match save_path.clone() {
			Some(path) => {
				if path.exists() {
					let result = File::open(path);
					match result {
						Ok(mut file) => {
							let _ = file.read(&mut ram);
						}
						Err(_) => {
							
						}
					}
				} 
			},
			None => (),
		}
		
        MBC2 {
            rom,
            ram,
            
            ram_enable: false,
            rom_bank_number: 0,

            save_path
        }
    }

	
}


impl Cartridge for MBC2 {
    
    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x3FFF => {
                self.rom[address]
            }
            0x4000..=0x7FFF => {
                let address = address - 0x4000;
                if self.rom_bank_number == 0 {
                    self.rom[address + ROM_BANK_SIZE]
                } else {
                    self.rom[address + (self.rom_bank_number) * ROM_BANK_SIZE]
                }

            }
            0xA000..=0xBFFF => {
                let address = (address - 0xA000) & 0x1FF;
                if self.ram_enable {
                    self.ram[address]
                } else {
                    0xFF
                }
            }
            _ => unreachable!("MBC2::read() at address: {:04X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x3FFF => {
				let bit_8 = (address >> 8) & 0x1;
				if bit_8 == 0 {
					if value & 0x0F == 0x0A {
						self.ram_enable = true;
					} else {
						self.ram_enable = false;
					}
				} else {
					let low_nibble = value & 0x0F;
					self.rom_bank_number = low_nibble as usize;
				}
			}
            0x4000..=0x7FFF => {
				()
            }
			0xA000..=0xBFFF => {
                let address = (address - 0xA000) & 0x1FF;
                if self.ram_enable {
                    self.ram[address] = value | 0xF0;
                }
            }
            _ => unreachable!("MBC2::write() at address: {:04X}", address),
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
                    eprintln!("MBC2: An error occured: {}", e);
                }
            }
        }
    }

	fn update_clock(&mut self) {
		
	}

	fn create_state(&self) -> String {
		serde_json::to_string(&self).unwrap()
	}

	fn load_state(&mut self, json_string: &str) {
		match serde_json::from_str::<MBC2>(json_string) {
            Ok(state) => {
				self.ram = state.ram.clone();
				self.ram_enable = state.ram_enable;
				self.rom_bank_number = state.rom_bank_number;
            },
            Err(e) => {
                eprintln!("Failed to deserialize state: {}", e);
            }
        }
	}
}
