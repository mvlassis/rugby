use super::Cartridge;
use super::ROM_BANK_SIZE;
use super::RAM_BANK_SIZE;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
enum BankingMode {
    Simple,
    Advanced,
}

#[derive(Serialize, Deserialize)]
pub struct MBC1 {
	#[serde(skip)]
    rom: Vec<u8>,
    ram: Vec<u8>,
    
    ram_enable: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
    banking_mode: BankingMode,

	rom_bit_mask: usize,

    save_path: Option<PathBuf>
}

impl MBC1 {
    pub fn new(data_buffer: &Vec<u8>, ram_banks: usize, save_path: Option<PathBuf>) -> Self {
        let mut rom: Vec<u8> = Vec::new();
        rom.extend_from_slice(data_buffer);
        let mut ram: Vec<u8> = Vec::new();
		let rom_banks = rom.len() / ROM_BANK_SIZE;
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
					ram = vec![0; ram_banks * RAM_BANK_SIZE];
				}
			},
			None => {
				ram = vec![0; ram_banks * RAM_BANK_SIZE];
			},
		}
		
        MBC1 {
            rom,
            ram,
            
            ram_enable: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
            banking_mode: BankingMode::Simple,
			rom_bit_mask,

            save_path
        }
    }

	
}


impl Cartridge for MBC1 {
    
    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x3FFF => {
                if self.banking_mode == BankingMode::Simple || self.rom_bit_mask <= 0x1F {
                    self.rom[address]
                } else {
					let rom_bank = self.ram_bank_number << 5;
                    self.rom[address + rom_bank * ROM_BANK_SIZE]
                }
            }
            0x4000..=0x7FFF => {
                let address = address - 0x4000;
				let rom_bank_number = match self.rom_bank_number {
					0 => 1,
					_ => self.rom_bank_number,
				};
				if self.rom_bit_mask > 0x1F {
					let rom_bank = (self.ram_bank_number << 5) + rom_bank_number & self.rom_bit_mask;
		            self.rom[address + (rom_bank * ROM_BANK_SIZE)]				
				}
				else {
					self.rom[address + (rom_bank_number & self.rom_bit_mask) * ROM_BANK_SIZE]	
				}

            }
            0xA000..=0xBFFF => {
                let address = address - 0xA000;
                if self.ram_enable {
                    if self.banking_mode == BankingMode::Advanced &&
						self.ram.len() / RAM_BANK_SIZE > 1  {
							self.ram[address + (self.ram_bank_number) * RAM_BANK_SIZE]
						} else {
							self.ram[address]
						}
                } else {
                    0xFF
                }

            }
            _ => unreachable!("MBC1::read() at address: {:04X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x1FFF => {
                let low_nibble = value & 0x0F;
                if low_nibble == 0xA {
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
            }
            0x2000..=0x3FFF => {
                let five_bits = value & 0x1F;
                self.rom_bank_number = five_bits as usize;
            }
            0x4000..=0x5FFF => {
                let two_bits = value & 0x03;
                self.ram_bank_number = two_bits as usize;
            }
            0x6000..=0x7FFF => {
                let one_bit = value & 0x01;
                if one_bit == 0 {
                    self.banking_mode = BankingMode::Simple;
                } else {
                    self.banking_mode = BankingMode::Advanced;
                }
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
					if self.banking_mode == BankingMode::Advanced &&
						self.ram.len() / RAM_BANK_SIZE > 1 {
							self.ram[address - 0xA000 + (self.ram_bank_number) * RAM_BANK_SIZE] = value;
						} else {
							self.ram[address - 0xA000] = value;
						}
                } else {
                    ()
                }

            }
            _ => unreachable!("MBC1::write() at address: {:04X}", address),
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
                    eprintln!("MBC1: An error occured: {}", e);
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
		match serde_json::from_str::<MBC1>(json_string) {
            Ok(state) => {
				self.ram = state.ram.clone();
				self.ram_enable = state.ram_enable;
				self.rom_bank_number = state.rom_bank_number;
				self.ram_bank_number = state.ram_bank_number;
				self.rom_bit_mask = state.rom_bit_mask;
            },
            Err(e) => {
                eprintln!("Failed to deserialize state: {}", e);
            }
        }
	}
}
