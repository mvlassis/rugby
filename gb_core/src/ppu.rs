use crate::color::Color;

const OAM_SEARCH_DOTS: u16 = 80;
const PIXEL_TRANSFER_DOTS: u16 = 172;
const HBLANK_DOTS: u16 = 204;
const LINE_DOTS: u16 = 456;

pub const GB_WIDTH: usize = 160;
pub const GB_HEIGHT: usize = 144;

#[derive(PartialEq)]
enum Mode {
	OAMSearch,
	PixelTransfer,
	HBlank,
	VBlank,
}

pub struct PPU {
	lcdc: u8,

	pub scy: u8,
	pub scx: u8,
	wy: u8,
	wx: u8,
		
	pub ly: u8,
	pub lyc: u8,
	pub stat: u8,
	
	pub bgp: u8,
	obp0: u8,
	obp1: u8,

	pub vram: [u8; 8192],
	oam_memory: [u8; 160],
	screen_buffer: [[Color; GB_WIDTH]; GB_HEIGHT],
	mode: Mode,
	current_clock: u16,
	pub vblank_interrupt: bool,
	pub stat_interrupt: bool,
	prev_interrupt_line: u8,
	pub frame_ready: bool,
}

impl PPU {
	pub fn new() -> Self {
		PPU {
			lcdc: 0,
			ly: 0,
			lyc: 0,
			stat: 0,
			scy: 0,
			scx: 0,
			wy: 0,
			wx: 0,
			
			bgp: 0,
			obp0: 0,
			obp1: 0,
			
			vram: [0; 8192],
			oam_memory: [0; 160],
			screen_buffer: [[Color::Black; GB_WIDTH]; GB_HEIGHT],
			mode: Mode::OAMSearch,
			current_clock: 0,
			vblank_interrupt: false,
			stat_interrupt: false,
			prev_interrupt_line: 0,
			frame_ready: false,
		}
	}

	pub fn initialize(&mut self) {
		self.lcdc = 0x91;
		self.stat = 0x85;
		self.bgp = 0xFC;
	}
	
	// Each dot lasts for 1 T-Cycle
	pub fn dot(&mut self) {
		match self.mode {
			Mode::OAMSearch => self.oam_search(),
			Mode::PixelTransfer => self.pixel_transfer(), 
			Mode::HBlank => self.hblank(),
			Mode::VBlank => self.vblank(),
		}
		self.update_stat();
	}

	pub fn get_screen_buffer(&self) -> &[[Color; GB_WIDTH]; GB_HEIGHT] {
		&self.screen_buffer
	}

	pub fn get_tilemap(&self) -> [[[Color; 8]; 8]; 384] {
		let mut tilemap = [[[Color::White; 8]; 8]; 384];

		for i in 0..384 {
			for j in 0..8 {
				let address = (i * 16 + j * 2) as u16;
				let byte1 = self.vram[address as usize];
				let byte2 = self.vram[address as usize + 1];

				for k in 0..8 {
					let color_id = ((byte2 >> (7 - k)) & 0x1) << 1;
					let color_id = color_id | ((byte1 >> (7 - k)) & 0x1);

					let color = match color_id {
						0 => Color::White,
						1 => Color::LightGray,
						2 => Color::DarkGray,
						3 => Color::Black,
						_ => unreachable!(),
					};

					tilemap[i][j][k] = color;
				}
			}
		}

		tilemap
	}

	// Gets a byte from VRAM
	pub fn get_vram(&self, address: usize) -> u8 {
		// TODO: Add Blocking
		// match self.mode {
		// 	Mode::OAMSearch | Mode::HBlank | Mode::VBlank => self.vram[address],
		// 	Mode::PixelTransfer => 0xFF,
		// }
		self.vram[address]
	}

	// Sets a byte in VRAM
	pub fn set_vram(&mut self, address: usize, value: u8) {
		// TODO: Add Blocking
		// match self.mode {
		// 	Mode::OAMSearch | Mode::HBlank | Mode::VBlank => self.vram[address] = value,
		// 	Mode::PixelTransfer => (),
		// }
		self.vram[address] = value;
	}
	
	fn oam_search(&mut self) {
		self.current_clock += 1;
		if self.current_clock >= OAM_SEARCH_DOTS-1 {
			self.mode = Mode::PixelTransfer;
		}		
	}

	fn pixel_transfer(&mut self) {
		self.current_clock += 1;
		if self.current_clock >= OAM_SEARCH_DOTS + PIXEL_TRANSFER_DOTS - 1 {
			self.mode = Mode::HBlank;
		}		
	}

	fn hblank(&mut self) {
		self.current_clock += 1;
		if self.current_clock == OAM_SEARCH_DOTS + PIXEL_TRANSFER_DOTS {
			self.draw_scanline();
		}
		if self.current_clock >= LINE_DOTS - 1 {
			self.ly += 1;
			self.current_clock = 0;
			if self.ly >= GB_HEIGHT as u8 {
				// We just entered vlank, request an interrupt
				self.mode = Mode::VBlank;
				self.vblank_interrupt = true;
				self.frame_ready = true;
			} else {
				self.mode = Mode::OAMSearch;
			}
		}		
	}

	fn vblank(&mut self) {
		self.current_clock += 1;
		if self.current_clock >= LINE_DOTS - 1 {
			self.ly += 1;
			self.current_clock = 0;
			if self.ly >= GB_HEIGHT as u8 + 10 {
				self.ly = 0;
				self.mode = Mode::OAMSearch;
			}
		}		
	}

	fn draw_scanline(&mut self) {
		let y = self.ly;

		for x in 0..GB_WIDTH as u8 {
			let color = self.get_pixel_color(x, y);
			self.screen_buffer[y as usize][x as usize] = color;
		}
	}

	fn get_pixel_color(&mut self, x: u8, y: u8) -> Color {
		// TODO: SCROLL
		let tile_map = PPU::get_bit(self.lcdc, 3);
		let bg_map_start = match tile_map {
			0 => 0x1800,
			_ => 0x1C00,
		};

		let tile_offset = 32 * (y.wrapping_add(self.scy) as u16 / 8) +
			(x.wrapping_add(self.scx) as u16) / 8;
		let tile_index_address = bg_map_start + tile_offset as usize;
		let tile_number = self.vram[tile_index_address] as usize;
		let tile_number_signed = self.vram[tile_index_address] as i16;

		let pixel_x = x as usize % 8;
		let pixel_y = y as usize % 8;
		let tile_address = match PPU::get_bit(self.lcdc, 4) {
			0 => (0x0800 + tile_number_signed * 16) as usize + pixel_y * 2,
			1 => (0x0000 + tile_number * 16) as usize + pixel_y * 2,
			_ => panic!("tile_address"),
		};

		let low_byte = self.vram[tile_address];
		let high_byte = self.vram[tile_address + 1];
		
		let color_index = ((high_byte >> (7 - pixel_x)) & 0x1) << 1 |
		((low_byte >> (7 - pixel_x)) & 0x1);

		let color = match color_index {
			0b00 => Color::Black,
			0b01 => Color::LightGray,
			0b10 =>	Color::DarkGray,
			0b11 =>	Color::White,
			_ => panic!("get_pixel_color()"),
		};
		color
	}

	// Updates the stat register
	fn update_stat(&mut self) {
		// Bits 0-1
		match self.mode {
			Mode::HBlank => {
				self.stat = PPU::set_bit(self.stat, 1, 0);
				self.stat = PPU::set_bit(self.stat, 0, 0);
			},
			Mode::VBlank => {
				self.stat = PPU::set_bit(self.stat, 1, 0);
				self.stat = PPU::set_bit(self.stat, 0, 1);
			},
			Mode::OAMSearch => {
				self.stat = PPU::set_bit(self.stat, 1, 1);
				self.stat = PPU::set_bit(self.stat, 0, 0);
			},
			Mode::PixelTransfer => {
				self.stat = PPU::set_bit(self.stat, 1, 1);
				self.stat = PPU::set_bit(self.stat, 0, 1);
			},
		}
		// Bit 2
		if self.ly == self.lyc {
			self.stat = PPU::set_bit(self.stat, 2, 1);
		} else {
			self.stat = PPU::set_bit(self.stat, 2, 0);
		}

		// Check for the STAT interrupt
		let mut curr_interrupt_line = 0;
		if PPU::get_bit(self.stat, 3) == 1 {
			if self.mode == Mode::HBlank {
				curr_interrupt_line |= 1;
			}
		}
		if PPU::get_bit(self.stat, 4) == 1 {
			if self.mode == Mode::VBlank {
				curr_interrupt_line |= 1;
			}
		}
		if PPU::get_bit(self.stat, 5) == 1 {
			if self.mode == Mode::OAMSearch {
				curr_interrupt_line |= 1;
			}
		}
		if PPU::get_bit(self.stat, 6) == 1 {
			if self.ly == self.lyc {
				curr_interrupt_line |= 1;
			}
		}
		if self.prev_interrupt_line == 0 && curr_interrupt_line == 1 {
			self.stat_interrupt = true;
		}
		// println!("Stat: {}{}{}{}", PPU::get_bit(self.stat, 6),
		// 		 PPU::get_bit(self.stat, 5), PPU::get_bit(self.stat, 4),
		// 		 PPU::get_bit(self.stat, 3));
		self.prev_interrupt_line = curr_interrupt_line
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
			_ => panic!("Set bit"),
		};
		new_value
	}
}
