use serde::{Serialize, Deserialize};
use crate::color::Color;
use crate::color::LogicalColor;
use crate::gb_mode::GBMode;
use crate::save_state::PPUState;

const OAM_SEARCH_DOTS: u16 = 80;
const PIXEL_TRANSFER_DOTS: u16 = 172;
// const HBLANK_DOTS: u16 = 204;
const LINE_DOTS: u16 = 456;

const VRAM_BANK_SIZE: usize = 8192;

pub const GB_WIDTH: usize = 160;
pub const GB_HEIGHT: usize = 144;

#[derive(Clone, Copy, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Mode {
    OAMSearch,
    PixelTransfer,
    HBlank,
    VBlank,
}

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct Object {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    attributes: u8,
}

pub struct PPU {
    pub lcdc: u8,

    pub ly: u8,
    pub lyc: u8,
    pub stat: u8,
    
    pub scy: u8,
    pub scx: u8,
    pub wy: u8,
    pub wx: u8,
    window_line_counter: u8,
    window_in_frame: bool,
    window_in_line: bool,
    
    object_buffer: Vec<Object>,
    
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub vram: [u8; 2 * VRAM_BANK_SIZE], // 2 banks
    oam: [u8; 160],
    pub screen_buffer: [[Color; GB_WIDTH]; GB_HEIGHT],
    mode: Mode,
    current_clock: u16,
    pub vblank_interrupt: bool,
    pub stat_interrupt: bool,
    prev_interrupt_line: u8,
    pub frame_ready: bool,
	pub ppu_disabled: bool,
	active_layers: [bool; 3], // 0: Background, 1: Window, 2: Objects

	// For Gameboy Color
	pub gb_mode: GBMode,
	pub vbk: u8, // VRAM bank
	bg_cram: [u8; 64], // Color RAM for the background
	pub bgpi: u8,
	obj_cram: [u8; 64], // Color RAM for the objects
	pub obpi: u8,
	pub opri: u8,
	pub entered_hblank: bool,
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
            window_line_counter: 0,
            window_in_frame: false,
            window_in_line: false,

            object_buffer: Vec::new(),
            
            bgp: 0,
            obp0: 0,
            obp1: 0,
            
            vram: [0; 2 * VRAM_BANK_SIZE],
            oam: [0; 160],
            screen_buffer: [[Color::Logical(LogicalColor::Black); GB_WIDTH]; GB_HEIGHT],
            mode: Mode::OAMSearch,
            current_clock: 0,
            vblank_interrupt: false,
            stat_interrupt: false,
            prev_interrupt_line: 0,
            frame_ready: false,
			ppu_disabled: false,
			active_layers: [true; 3],

			gb_mode: GBMode::DMG,
			vbk: 0,
			bg_cram: [0xFF; 64],
			bgpi: 0xC8,
			obj_cram: [0xFF; 64],
			obpi: 0xD0,
			opri: 0x00,
			entered_hblank: false,
        }
    }

    // Initializes the fields
    pub fn initialize(&mut self, gb_mode: GBMode) {
        self.lcdc = 0x91;
        self.stat = 0x85;
		self.scy = 0;
		self.scx = 0;
		self.ly = 0;
		self.lyc = 0;
        self.bgp = 0xFC;
		self.wy = 0;
		self.wx = 0;
		self.vbk = 0;

		self.gb_mode = gb_mode;
		self.bg_cram = [0xFF; 64];
		self.bgpi = 0xC8;
		self.obj_cram = [0xFF; 64];
		self.obpi = 0xD0;
		self.opri = 0x00; // TODO Monochrome games in GBC
		self.entered_hblank = false;
    }
    
    // Each dot lasts for 1 T-Cycle
    pub fn dot(&mut self) {
		self.update_clock();
		if PPU::get_bit(self.lcdc, 7) == 0 && self.ppu_disabled == false {
			self.ppu_disabled = true;
		}
		if PPU::get_bit(self.lcdc, 7) == 1 && self.ppu_disabled == true {
			self.ppu_disabled = false;
			self.ly = 0;
			self.current_clock = 1;
			self.window_line_counter = 0;
			self.window_in_frame = false;
			self.mode = Mode::OAMSearch;
		}
        match self.mode {
            Mode::OAMSearch => self.oam_search(),
            Mode::PixelTransfer => self.pixel_transfer(), 
            Mode::HBlank => self.hblank(),
            Mode::VBlank => self.vblank(),
        }
		if self.ppu_disabled == false {
			self.update_stat();
        } 
    }

	// Updates the PPU internal clock
	fn update_clock(&mut self) {
		self.current_clock += 1;
		match self.mode {
			Mode::OAMSearch => {
				if self.current_clock >= OAM_SEARCH_DOTS-1 {
					self.mode = Mode::PixelTransfer;
				}
			},
			Mode::PixelTransfer => {
				if self.current_clock >= OAM_SEARCH_DOTS + PIXEL_TRANSFER_DOTS - 1 {
					self.mode = Mode::HBlank;
					self.entered_hblank = true;
				}   
			},
			Mode::HBlank => {
				if self.current_clock >= LINE_DOTS - 1 {
					self.ly += 1;
					self.current_clock = 0;
					if self.ly >= GB_HEIGHT as u8 {
						// We just entered VBlank, request an interrupt
						self.mode = Mode::VBlank;
						self.frame_ready = true;
						if self.ppu_disabled == false {
							self.vblank_interrupt = true;
						}
					} else {
						self.mode = Mode::OAMSearch;
					}
				} 
			},
			Mode::VBlank => {
				if self.current_clock >= LINE_DOTS - 1 {
					self.ly += 1;
					self.current_clock = 0;
					if self.ly >= GB_HEIGHT as u8 + 10 {
						self.ly = 0;
						self.window_line_counter = 0;
						self.window_in_frame = false;
						self.mode = Mode::OAMSearch;
					}
				}  
			},
		}

	}
	
    // Returns the screen buffer
    pub fn get_screen_buffer(&self) -> &[[Color; GB_WIDTH]; GB_HEIGHT]{
        &self.screen_buffer
    }

    // Returns the tilemap
    pub fn get_tilemap(&self) -> [[[Color; 8]; 8]; 384] {
        let mut tilemap = [[[Color::Logical(LogicalColor::White); 8]; 8]; 384];
        for i in 0..384 {
            for j in 0..8 {
                let address = (i * 16 + j * 2) as u16;
                let byte1 = self.vram[address as usize];
                let byte2 = self.vram[address as usize + 1];

                for k in 0..8 {
                    let color_id = ((byte2 >> (7 - k)) & 0x1) << 1;
                    let color_id = color_id | ((byte1 >> (7 - k)) & 0x1);

                    let color = match color_id {
						0b00 => Color::Logical(LogicalColor::White),
						0b01 => Color::Logical(LogicalColor::LightGray),
						0b10 => Color::Logical(LogicalColor::DarkGray),
						0b11 => Color::Logical(LogicalColor::Black),
                        _ => unreachable!("PPU::get_tilemap()"),
                    };

                    tilemap[i][j][k] = color;
                }
            }
        }
        tilemap
    }

    // Returns the bg map
    pub fn get_bg_map(&self, map: u8) -> [[[Color; 8]; 8]; 1024] {
        let mut tilemap = [[[Color::Logical(LogicalColor::White); 8]; 8]; 1024];
        let bg_map_start = match (map >> 0) & 0x1 {
            0 => 0x1800,
            1 => 0x1C00,
            _ => unreachable!(),
        };
        for tile in 0..1024 {
            let tile_address = self.vram[bg_map_start + tile] as usize;
            let tile_address_signed = i16::from(self.vram[bg_map_start + tile] as i8) + 128;
            for y in 0..8 {
                let correct_tile = match (map >> 1) & 0x1 {
                    0 => tile_address * 16 + 2 * y,
                    1 => (0x0800 + tile_address_signed * 16) as usize + 2 * y,
                    _ => unreachable!(),
                };
                let low_byte = self.vram[correct_tile];
                let high_byte  = self.vram[correct_tile + 1];
                for x in 0..8 {
                    let color_index = ((high_byte >> (7 - x)) & 0x1) << 1 |
                    ((low_byte >> (7 - x)) & 0x1);

                    let color = match color_index {
						0b00 => Color::Logical(LogicalColor::White),
						0b01 => Color::Logical(LogicalColor::LightGray),
						0b10 => Color::Logical(LogicalColor::DarkGray),
						0b11 => Color::Logical(LogicalColor::Black),
                        _ => unreachable!(),
                    };
                    tilemap[tile][y][x] = color;
                }
            }
        }
        tilemap
    }

    // Gets a byte from VRAM
    pub fn get_vram(&self, address: usize) -> u8 {
		// TODO: Blocking
        // match self.mode {
        //  Mode::OAMSearch | Mode::HBlank | Mode::VBlank => self.vram[address],
        //  Mode::PixelTransfer => 0xFF,
        // };
		self.vram[self.vbk as usize * VRAM_BANK_SIZE + address]
    }

    // Sets a byte in VRAM
    pub fn set_vram(&mut self, address: usize, value: u8) {
		// TODO: Blocking
        // match self.mode {
        //  Mode::OAMSearch | Mode::HBlank | Mode::VBlank => self.vram[address] = value,
        //  Mode::PixelTransfer => (),
        // }
        self.vram[self.vbk as usize * VRAM_BANK_SIZE + address] = value;
    }

    // Gets a byte from OAM
    pub fn get_oam(&self, address: usize) -> u8 {
        // TODO: Add Blocking
        // match self.mode {
        //  Mode::OAMSearch | Mode::HBlank | Mode::VBlank => self.vram[address],
        //  Mode::PixelTransfer => 0xFF,
        // }
        self.oam[address]
    }

    // Sets a byte in OAM
    pub fn set_oam(&mut self, address: usize, value: u8) {
        // TODO: Add Blocking
        // match self.mode {
        //  Mode::OAMSearch | Mode::HBlank | Mode::VBlank => self.vram[address] = value,
        //  Mode::PixelTransfer => (),
        // }
        self.oam[address] = value;
    }

	pub fn set_palette(&mut self, address: usize, value: u8) {
		match address {
			0xFF69 => {
				let auto_increment = PPU::get_bit(self.bgpi, 7) == 1;
				let address = self.bgpi & 0x3F;
				if auto_increment {
					self.bgpi = (self.bgpi & 0x80) | (address + 1);
				}
				self.bg_cram[address as usize] = value;
			},
			0xFF6B => {
				let auto_increment = PPU::get_bit(self.obpi, 7) == 1;
				let address = self.obpi & 0x3F;
				if auto_increment {
					self.obpi = (self.obpi & 0x80) | (address + 1);
				}
				self.obj_cram[address as usize] = value;
			}
			_ => unreachable!(),
		}
	}
	
    // Mode 2
    fn oam_search(&mut self) {
        if self.current_clock == 1 {
            self.scan_objects();
        }
    }

    // Searches for objects in OAM memory and saves them to the object buffer
    fn scan_objects(&mut self) {
        let sprite_mode = PPU::get_bit(self.lcdc, 2);
        
        self.object_buffer = Vec::new();
        for i in 0..40 { // 40 objects in OAM
            let y_position = self.oam[4 * i];
            let x_position = self.oam[4 * i + 1];
            let tile_index = self.oam[4 * i + 2];
            let attributes = self.oam[4 * i + 3];

            let sprite_height = match sprite_mode {
                0 => 8,
                1 => 16,
                _ => unreachable!("PPU::scan_objects()"),
            };
            if x_position > 0 && y_position <= self.ly + 16
                && (y_position + sprite_height) > self.ly + 16
                && self.object_buffer.len() < 10 {
                    let obj = Object {
                        y_position,
                        x_position,
                        tile_index,
                        attributes,
                    };
                    self.object_buffer.push(obj);
                }
        }
    }

    // Mode 3
    fn pixel_transfer(&mut self) {
    
    }

    // Mode 0
    fn hblank(&mut self) {
        if self.current_clock == OAM_SEARCH_DOTS + PIXEL_TRANSFER_DOTS {
            self.draw_scanline();
        } 
    }

    // Mode 1
    fn vblank(&mut self) {
   
    }

    // Draw all pixels in the currect line
    fn draw_scanline(&mut self) {
        let y = self.ly;
        if  PPU::get_bit(self.lcdc, 5) == 1 && self.window_in_frame == false
            && y == self.wy {
                self.window_in_frame = true;
                self.window_line_counter = 0;
            }
        self.window_in_line = false;
        

        for x in 0..GB_WIDTH as u8 {
            let (bg_color_bits, bg_attributes) = self.get_bg_bits(x, y);
			let obj_result = if PPU::get_bit(self.lcdc, 1) == 1 && self.active_layers[2] == true {
				self.get_sprite_bits(x, y)
			} else {
				None
			};

			let mut final_color = self.get_final_color(bg_color_bits, bg_attributes, obj_result);
				
            if PPU::get_bit(self.lcdc, 7) == 0 {
                final_color = Color::Logical(LogicalColor::White);
            }
            self.screen_buffer[y as usize][x as usize] = final_color;
        }

        if self.window_in_line {
            self.window_line_counter = self.window_line_counter.wrapping_add(1);
        }
    }

	fn get_final_color(&self, bg_color_bits: u8, bg_attributes: u8,
					   obj_result: Option<(u8, u8)>) -> Color {
		match self.gb_mode {
			GBMode::DMG => {
				let color = match obj_result {
					None => self.get_bgp_color(bg_color_bits, 0),
					Some((obj_color_bits, obj_attributes)) => {
						let priority = PPU::get_bit(obj_attributes, 7);
						if bg_color_bits == 0 || priority == 0 {
							match PPU::get_bit(obj_attributes, 4) {
								0 => self.get_obp_color(obj_color_bits, obj_attributes),
								1 => self.get_obp_color(obj_color_bits, obj_attributes),
								_ => unreachable!("PPU:get_final_color()"),
							}
						} else {
							self.get_bgp_color(bg_color_bits, 0)
						}
					},
				};
				color
			},
			GBMode::CGB => {
				let bg_palette = bg_attributes & 0b111;
				let color = match obj_result {
					None => self.get_bgp_color(bg_color_bits, bg_palette),
					Some((obj_color_bits, obj_attributes)) => {
						if PPU::get_bit(self.lcdc, 0) == 0 || bg_color_bits == 0 {
							self.get_obp_color(obj_color_bits, obj_attributes)
						} else if PPU::get_bit(bg_attributes, 7) == 0 && PPU::get_bit(obj_attributes, 7) == 0 {
								self.get_obp_color(obj_color_bits, obj_attributes)
						} else {
							self.get_bgp_color(bg_color_bits, bg_palette)
						}
					}
				};
				color

			}
		}
	}
	
    // Returns the sprite color (if any) for the current pixel
    fn get_sprite_bits(&mut self, x: u8, y: u8) -> Option<(u8, u8)> {
        // Firstly we select all sprites that have pixels in this x position
        let mut candidate_sprites: Vec<Object> = self.object_buffer.clone().into_iter()
					.filter(|obj| obj.x_position <= x + 8 &&
							obj.x_position > x).collect();
		if self.gb_mode == GBMode::DMG {
			candidate_sprites.sort_by(|a, b| a.x_position.cmp(&b.x_position));
		}

        let mut current_obj = 0;

        // Iterate all candidate objects and find the first non-transparent pixel
        while current_obj < candidate_sprites.len() {
            let object = candidate_sprites[current_obj];
            let mut pixel_x = (x + 8) - object.x_position;
            if PPU::get_bit(object.attributes, 5) == 1 {
                pixel_x = 7 - pixel_x;
            }
            let mut pixel_y = (y + 16) - object.y_position;
            if PPU::get_bit(object.attributes, 6) == 1 {
                if PPU::get_bit(self.lcdc, 2) == 1 {
                    pixel_y = 15 - pixel_y;                 
                } else {
                    pixel_y = 7 - pixel_y;
                }
            }

            let tile_address = if PPU::get_bit(self.lcdc, 2) == 0 {
                16 * object.tile_index as usize + 2 * pixel_y as usize
            } else {
                16 * (object.tile_index & 0xFE) as usize + 2 * pixel_y as usize 
            };

			let (low_byte, high_byte) = match self.gb_mode {
				GBMode::DMG => (self.vram[tile_address], self.vram[tile_address+1]),
				GBMode::CGB => {
					let bank = PPU::get_bit(object.attributes, 3) as usize;
					(self.vram[VRAM_BANK_SIZE * bank + tile_address], self.vram[VRAM_BANK_SIZE * bank + tile_address + 1])
				}
			};
			
            let color_index = ((high_byte >> (7 - pixel_x)) & 0x1) << 1 |
            ((low_byte >> (7 - pixel_x)) & 0x1);

            // 0b00 for objects means transparent pixel
            if color_index == 0b00 {
                current_obj += 1;
                continue;
            } else {
				return Some((color_index, object.attributes));
            }
        }
        None
    }

	// Returns the background color bits and the tile's attributes
    fn get_bg_bits(&mut self, x: u8, y: u8) -> (u8, u8) {
        // Disable background in DMG mode if LCDC bit 0 == 0
        if PPU::get_bit(self.lcdc, 0) == 0 && self.gb_mode == GBMode::DMG {
            return (0, 0);
        }

        let window_enabled = PPU::get_bit(self.lcdc, 5) == 1 && self.window_in_frame
            && self.wx <= 166 && (x + 7) >= self.wx && self.wy <= y && self.wy <= 143;
        if window_enabled {
            self.window_in_line = true
        }
		let window_enabled = window_enabled && self.active_layers[1];
		if !window_enabled && !self.active_layers[0] {
			return (0, 0);
		}
		
        // Get the correct tilemap
        let tile_map = match window_enabled {
            true => PPU::get_bit(self.lcdc, 6),
            false => PPU::get_bit(self.lcdc, 3),
        };
        let bg_map_start = match tile_map {
            0 => 0x1800,
            1 => 0x1C00,
            _ => unreachable!("PPU::get_pixel_color(), bg_map"),
        };

        let tile_row = match window_enabled {
            true => 32 * (self.window_line_counter  as u16 / 8),
            false => 32 * (y.wrapping_add(self.scy) as u16 / 8),
        };
        let tile_column = match window_enabled {
            true => ((x.wrapping_sub(self.wx.wrapping_sub(7)))/8) & 0x1F,
            false => ((x.wrapping_add(self.scx)/8)) & 0x1F,
        };

        let tile_index_address = bg_map_start + tile_row as usize + tile_column as usize;
        let tile_number = self.vram[tile_index_address] as usize;
        let tile_number_signed = self.vram[tile_index_address] as i8 as i16;

		let bg_attributes = self.vram[VRAM_BANK_SIZE + tile_index_address];
		
        let mut pixel_x = if window_enabled {
            x.wrapping_sub(self.wx.wrapping_sub(7)) as usize % 8
        } else {
            x.wrapping_add(self.scx) as usize % 8
        };
        let mut pixel_y = if window_enabled {
            y.wrapping_sub(self.wy) as usize % 8
        } else {
            y.wrapping_add(self.scy) as usize % 8
        };

		if self.gb_mode == GBMode::CGB {
            if PPU::get_bit(bg_attributes, 5) == 1 {
                pixel_x = 7 - pixel_x;
            }
            if PPU::get_bit(bg_attributes, 6) == 1 {
				pixel_y = 7 - pixel_y;
            }
		}
		
        let tile_address = match PPU::get_bit(self.lcdc, 4) {
            0 => (0x1000 + (tile_number_signed * 16)) as usize + (pixel_y * 2),
            1 => (0x0000 + tile_number * 16) as usize + (pixel_y * 2),
            _ => unreachable!("PPU::get_pixel_color(), tile_address"),
        };

        let (low_byte, high_byte) = match self.gb_mode {
			GBMode::DMG => (self.vram[tile_address], self.vram[tile_address+1]),
			GBMode::CGB => {
				let bank = PPU::get_bit(bg_attributes, 3) as usize;
				(self.vram[VRAM_BANK_SIZE * bank + tile_address], self.vram[VRAM_BANK_SIZE * bank + tile_address + 1])
			}
		};
        
        let color_index = ((high_byte >> (7 - pixel_x)) & 0x1) << 1 |
        ((low_byte >> (7 - pixel_x)) & 0x1);
		
        (color_index, bg_attributes)
    }

    // Get the proper color from background palettes
    fn get_bgp_color(&self, index: u8, color_palette: u8) -> Color {
		match self.gb_mode {
			GBMode::DMG => {
				let shift_amount = index * 2;
				let color_bits = (self.bgp >> shift_amount) & 0b11;

				match color_bits {
					0b00 => Color::Logical(LogicalColor::White),
					0b01 => Color::Logical(LogicalColor::LightGray),
					0b10 => Color::Logical(LogicalColor::DarkGray),
					0b11 => Color::Logical(LogicalColor::Black),
					_ => unreachable!("PPU::get_bgp_color()"), 
				}
			},
			GBMode::CGB => {
				let palette_address = 8 * color_palette + 2 * index;
				let low_byte = self.bg_cram[palette_address as usize];
				let high_byte = self.bg_cram[palette_address as usize + 1];
				let full_byte = ((high_byte as u16) << 8) | low_byte as u16;
				Color::RGB(full_byte)
			}
		}

    }

	// Get the proper color from object palettes
	fn get_obp_color(&self, index: u8, obj_attributes: u8) -> Color {
		match self.gb_mode {
			GBMode::DMG => {
				let shift_amount = index * 2;
				let color_bits = match PPU::get_bit(obj_attributes, 4) {
					0 => (self.obp0 >> shift_amount) & 0b11,
					_ => (self.obp1 >> shift_amount) & 0b11,
				};
				match color_bits {
					0b00 => Color::Logical(LogicalColor::White),
					0b01 => Color::Logical(LogicalColor::LightGray),
					0b10 => Color::Logical(LogicalColor::DarkGray),
					0b11 => Color::Logical(LogicalColor::Black),
					_ => unreachable!("PPU::get_obp_color()"), 
				}
			},
			GBMode::CGB => {
				let color_palette = obj_attributes & 0x07;
				let palette_address = 8 * color_palette + 2 * index;
				let low_byte = self.obj_cram[palette_address as usize];
				let high_byte = self.obj_cram[palette_address as usize + 1];
				let full_byte = ((high_byte as u16) << 8) | low_byte as u16;
				Color::RGB(full_byte)
			},
		}
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
            _ => unreachable!("PPU::set_bit()"),
        };
        new_value
    }

	// Toggles the visibility of a given layer 
	pub fn toggle_layer(&mut self, i: usize) {
		self.active_layers[i] = !self.active_layers[i];
	}
	
	// Creates a PPUState from the PPU
	pub fn create_state(&self) -> PPUState {
		PPUState {
			lcdc: self.lcdc,
			ly: self.ly,
			lyc: self.lyc,
			stat: self.stat,
			scy: self.scy,
			scx: self.scx,
			wy: self.wy,
			wx: self.wx,
			window_line_counter: self.window_line_counter,
			window_in_frame: self.window_in_frame,
			window_in_line: self.window_in_line,
			object_buffer: self.object_buffer.clone(),
			bgp: self.bgp,
			obp0: self.obp0,
			obp1: self.obp1,
			vram: self.vram.clone(),
			oam: self.oam.clone(),
			// screen_buffer: vec![vec![Color::Black; 1]; 5],
			// screen_buffer: self.screen_buffer.iter()
			// 	.flat_map(|row| row.iter().copied()).collect(),
			mode: self.mode,
			current_clock: self.current_clock,
			vblank_interrupt: self.vblank_interrupt,
			stat_interrupt: self.stat_interrupt,
			prev_interrupt_line: self.prev_interrupt_line,
			frame_ready: self.frame_ready,
			ppu_disabled: self.ppu_disabled,

			gb_mode: self.gb_mode,
			vbk: self.vbk,
			bg_cram: self.bg_cram.clone(),
			bgpi: self.bgpi,
			obj_cram: self.obj_cram.clone(),
			obpi: self.obpi,
			opri: self.opri,
			entered_hblank: self.entered_hblank,
		}
	}

	// Loads a PPUState to the PPU
	pub fn load_state(&mut self, ppu_state: PPUState) {
		self.lcdc = ppu_state.lcdc;
		self.ly = ppu_state.ly;
		self.lyc = ppu_state.lyc;
		self.stat = ppu_state.stat;
		self.scy = ppu_state.scy;
		self.scx = ppu_state.scx;
		self.wy = ppu_state.wy;
		self.wx = ppu_state.wx;
		self.window_line_counter = ppu_state.window_line_counter;
		self.window_in_frame = ppu_state.window_in_frame;
		self.window_in_line = ppu_state.window_in_line;
		self.object_buffer = ppu_state.object_buffer.clone();
		self.bgp = ppu_state.bgp;
		self.obp0 = ppu_state.obp0;
		self.obp1 = ppu_state.obp1;
		self.vram = ppu_state.vram.clone();
		self.oam = ppu_state.oam.clone();
		self.mode = ppu_state.mode;
		self.current_clock = ppu_state.current_clock;
		self.vblank_interrupt = ppu_state.vblank_interrupt;
		self.stat_interrupt = ppu_state.stat_interrupt;
		self.prev_interrupt_line = ppu_state.prev_interrupt_line;
		self.frame_ready = ppu_state.frame_ready;
		self.ppu_disabled = ppu_state.ppu_disabled;

		self.gb_mode = ppu_state.gb_mode;
		self.vbk = ppu_state.vbk;
		self.bg_cram = ppu_state.bg_cram.clone();
		self.bgpi = ppu_state.bgpi;
		self.obj_cram = ppu_state.obj_cram.clone();
		self.obpi = ppu_state.obpi;
		self.opri = ppu_state.opri;
		self.entered_hblank = ppu_state.entered_hblank;
	}
}
