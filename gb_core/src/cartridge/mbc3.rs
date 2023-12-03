use super::Cartridge;
use super::BANK_SIZE;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

struct Clock {
	seconds: u8,
	minutes: u8,
	hours: u8,
	days_l: u8,
	days_h: u8,

	base: u64,
	save_path: Option<PathBuf>,
}

impl Clock {

	pub fn new(save_path: Option<PathBuf>) -> Self {
		let mut base = 0;
        let mut byte_array = [0; 8];
		let mut buffer: Vec<u8> = Vec::new();

		match save_path.clone() {
			Some(path) => {
				if path.exists() {
					let result = File::open(path);
					match result {
						Ok(mut file) => {
							let _ = file.read_to_end(&mut buffer);
							byte_array.copy_from_slice(&buffer);
							base = u64::from_be_bytes(byte_array);
						}
						Err(_) => {}
					}
				} 
			},
			None => (),
		}
		
        Clock {
            seconds: 0,
			minutes: 0,
			hours: 0,
			days_l: 0,
			days_h: 0,
			
			base,
            save_path
        }
    }

	pub fn increment_seconds(&mut self) {
		if get_bit(self.days_h, 6) == 0 {
			self.base += 1;
		}
	}

	pub fn update_base(&mut self) {
		let days = (self.days_h as u64 & 0x01) << 8 | (self.days_l as u64);
		self.base = self.seconds as u64 + 60 * self.minutes as u64 +
			3600 * self.hours as u64 + 3600 * 24 * days;
	}
	
	pub fn update(&mut self) {
		let halt_bit = get_bit(self.days_h, 6);
		if halt_bit == 1 {
			return;
		}
		self.seconds = (self.base % 60) as u8;
		let minutes = self.base / 60;
		self.minutes = (minutes % 60) as u8;
		let hours = minutes / 60;
		self.hours = (hours % 24) as u8;
		let days = hours / 24;
		self.days_l = (days % 256) as u8;
		match days {
			0x0000..=0x00FF => (),
			0x0100..=0x01FF => {
				self.days_h = set_bit(self.days_h, 0, 1);
			},
			_ => {
				self.days_h = set_bit(self.days_h, 0, 0);
				self.days_h = set_bit(self.days_h, 7, 1);
			}
		}
	}

	pub fn save(&mut self) {
        if let Some(save_path) = &self.save_path {
			let result = File::create(&save_path);
            match result {
                Ok(mut file) => {
                    let _ = file.write_all(&self.base.to_be_bytes());
                },
                Err(e) => {
                    eprintln!("MBC3: An error occured: {}", e);
                }
            }
        }
    }
}

// Get bit at a specific position
fn get_bit(value: u8, bit_position: u8) -> u8 {
	let bit = (value >> bit_position) & 0x1;
	bit as u8
}

// Set bit at a specific position to a specific bit value
fn set_bit(value: u8, bit_position: u8, bit_value: u8) -> u8 {
	let new_value = match bit_value {
		0 => value & !(1 << bit_position),
		1 => value | 1 << bit_position,
		_ => unreachable!("MBC3::set_bit()"),
	};
	new_value
}

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    
    ram_timer_enable: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
	clock: Option<Clock>,
	latch_clock_00: bool, // True if the prvious value written to latch clock
	// register was 0x00

    save_path: Option<PathBuf>
}

impl MBC3 {
    pub fn new(data_buffer: &Vec<u8>, ram_banks: usize, save_path: Option<PathBuf>, has_clock: bool) -> Self {
        let mut rom: Vec<u8> = Vec::new();
        rom.extend_from_slice(data_buffer);
        let mut ram: Vec<u8> = Vec::new();
		let mut clock = match has_clock {
			true => Some(Clock::new(None)),
			false => None,
		};

		match save_path.clone() {
			Some(path) => {
				if path.exists() {
					let mut clock_path = PathBuf::from(path.clone());
					if let Some(file_stem) = clock_path.file_stem() {
						clock_path = clock_path.with_file_name(file_stem).with_extension("rtc");
					}
					clock = Some(Clock::new(Some(clock_path)));
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
		
        MBC3 {
            rom,
            ram,
            
            ram_timer_enable: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
			clock,
			latch_clock_00: false,

            save_path
        }
    }
	
}


impl Cartridge for MBC3 {
    
    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x3FFF => {
                self.rom[address]
            }
            0x4000..=0x7FFF => {
                let address = address - 0x4000;
                if self.rom_bank_number == 0 {
                    self.rom[address + BANK_SIZE]
                } else {
                    self.rom[address + (self.rom_bank_number) * BANK_SIZE]
                }

            }
            0xA000..=0xBFFF => {
                let address = address - 0xA000;
                if self.ram_timer_enable {
                    if self.ram_bank_number <= 0x03 {
                        self.ram[address + (self.ram_bank_number) * BANK_SIZE]
                    } else if let Some(clock) = &self.clock  {
						if self.ram_bank_number <= 0x0C {
							match self.ram_bank_number {
								0x08 => clock.seconds,
								0x09 => clock.minutes,
								0x0A => clock.hours,
								0x0B => clock.days_l,
								0x0C => clock.days_h,
								_ => unreachable!("MBC3::read()"),
							}
						} else { 0xFF }

                    } else { 0xFF }

                } else { 0xFF }

            }
            _ => unreachable!("MBC3::read() at address: {:04X}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x1FFF => {
                let low_nibble = value & 0x0F;
                if low_nibble == 0x0A {
                    self.ram_timer_enable = true;
                } else {
                    self.ram_timer_enable = false;
                }
            }
            0x2000..=0x3FFF => {
                let seven_bits = value & 0x7F;
                self.rom_bank_number = seven_bits as usize;
            }
            0x4000..=0x5FFF => {
                let four_bits = value & 0x0F;
                self.ram_bank_number = four_bits as usize;
            }
            0x6000..=0x7FFF => {
                if value == 0x01 && self.latch_clock_00 {
					self.latch_clock_00 = false;
					if let Some(clock) = self.clock.as_mut() {
						clock.update();
					}
				} else {
					if value == 0x00 {
						self.latch_clock_00 = true;
					}
				}
				
            }
            0xA000..=0xBFFF => {
				let address = address - 0xA000;
                if self.ram_timer_enable {
                    if self.ram_bank_number <= 0x03 {
                        self.ram[address + (self.ram_bank_number) * BANK_SIZE] = value;
                    } else if let Some(clock) = self.clock.as_mut() {
						if self.ram_bank_number <= 0x0C {
							match self.ram_bank_number {
								0x08 => clock.seconds = value,
								0x09 => clock.minutes = value,
								0x0A => clock.hours = value,
								0x0B => clock.days_l = value,
								0x0C => clock.days_h = value,
								_ => unreachable!("MBC3::read()"),
							}
						}
						clock.update_base();
                    } 
                } else {
                    ()
                }

            }
            _ => unreachable!("MBC3::write() at address: {:04X}", address),
        }
    }

	fn save(&mut self) {
		if let Some(clock) = self.clock.as_mut() {
			clock.save();
		}

        if let Some(save_path) = &self.save_path {
			let result = File::create(&save_path);
            match result {
                Ok(mut file) => {
                    let _ = file.write_all(&self.ram);
                },
                Err(e) => {
                    eprintln!("MBC3: An error occured: {}", e);
                }
            }
        }
    }

	fn update_clock(&mut self) {
		if let Some(clock) = self.clock.as_mut() {
			clock.increment_seconds();
		}
	}
}
