use super::Cartridge;

const ROM_SIZE: usize = 32768;

pub struct RomOnly {
    rom: [u8; ROM_SIZE],
}

impl RomOnly {
    pub fn new(data_buffer: &Vec<u8>) -> Self {
        let end = data_buffer.len();
        let mut rom = [0; ROM_SIZE];
        rom[0..end].copy_from_slice(data_buffer);
        RomOnly {
            rom
        }
    }
}

impl Cartridge for RomOnly {
    
    fn read(&self, address: u16) -> u8 {
        if address >= 0xA000 {
            0xFF // External RAM doesn't exist
        } else {
            self.rom[address as usize]
        }

    }

    fn write(&mut self, _address: u16, _value: u8) {
        () // We ignore all writes to ROM
    }

	fn save(&mut self) {
		
	}
}
