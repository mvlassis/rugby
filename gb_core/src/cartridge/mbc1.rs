use super::Cartridge;

const BANK_SIZE: usize = 16384;

#[derive(PartialEq)]
enum BankingMode {
    Rom,
    Ram,
}

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    
    ram_enable: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
    ram_banks: usize,
    banking_mode: BankingMode,
}

impl MBC1 {
    pub fn new(data_buffer: &Vec<u8>, ram_banks: usize) -> Self {
        let mut rom: Vec<u8> = Vec::new();
        rom.extend_from_slice(data_buffer);
        let ram: Vec<u8> = Vec::new();
        MBC1 {
            rom,
            ram,
            
            ram_enable: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
            ram_banks,
            banking_mode: BankingMode::Rom,
        }
    }
}

impl Cartridge for MBC1 {
    
    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x3FFF => {
                if self.banking_mode == BankingMode::Rom {
                    self.rom[address]
                } else {
                    self.rom[address]
                }
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
                if self.ram_enable {
                    self.ram[address - 0xA000]
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
                println!("Ram bank");
                let two_bits = value & 0x03;
                self.ram_bank_number = two_bits as usize;
            }
            0x6000..=0x7FFF => {
                let one_bit = value & 0x01;
                if one_bit == 0 {
                    self.banking_mode = BankingMode::Rom;
                } else {
                    println!("Changing mode to RAM");
                    self.banking_mode = BankingMode::Ram;
                }
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram[address - 0xA000] = value;
                } else {
                    ()
                }

            }
            _ => unreachable!("MBC1::write() at address: {:04X}", address),
        }
    }
}
