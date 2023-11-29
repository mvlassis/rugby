use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use sdl2::TimerSubsystem;

use gb_core::color::Color as LogicalColor;

const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;

// Holds all information needed for drawing to the screen
pub struct VideoDriver {
	screen_width: usize,
	screen_height: usize,
	canvas: Canvas<Window>,
	canvas_tilemap: Canvas<Window>,
	canvas_bg_map: Canvas<Window>,
	
	window_main: Window,
	window_tilemap: Window,
	window_bg_map: Window,

	scale: u32,
	palette: [Color; 4],

	video_subsystem: VideoSubsystem,
	timer_subsystem: TimerSubsystem,
	start: u64,
}

impl VideoDriver {
	pub fn new(sdl: &sdl2::Sdl, scale: u32) -> Self {
		sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "0");
		let window_width = (GB_WIDTH as u32) * scale;
		let window_height = (GB_HEIGHT as u32) * scale;

		let video_subsystem = sdl.video().unwrap();
		let timer_subsystem = sdl.timer().unwrap();
		let start = timer_subsystem.performance_counter();


		let window_tilemap = video_subsystem.window("Tilemap", 128 * scale, 192 * scale)
			.position(1250, 300).opengl().build().unwrap();
		let mut canvas_tilemap = window_tilemap.into_canvas().build().unwrap();

		let window_bg_map = video_subsystem.window("BGMap", 256 * scale, 256 * scale)
			.position(100, 150).opengl().build().unwrap();
		let mut canvas_bg_map = window_bg_map.into_canvas().build().unwrap();
		
		let window_main = video_subsystem.window("Rugby", window_width, window_height).position_centered().opengl().build().unwrap();
		let mut canvas = window_main.into_canvas().build().unwrap();

		let palette = [VideoDriver::hex_to_rgb("#d0d058"),
					   VideoDriver::hex_to_rgb("#a0a840"),
					   VideoDriver::hex_to_rgb("#708028"),
					   VideoDriver::hex_to_rgb("#405010")];
		
		canvas.clear();
		canvas.present();
		canvas_tilemap.clear();
		canvas_tilemap.present();
		canvas_bg_map.clear();
		canvas_bg_map.present();
		VideoDriver {
			screen_width: GB_WIDTH,
			screen_height: GB_HEIGHT,
			canvas,
			canvas_tilemap,
			canvas_bg_map,

			window_main,
			window_tilemap,
			window_bg_map,
			
			scale,
			palette,
			video_subsystem,
			timer_subsystem,
			start,
		}
	}

	pub fn draw_window(&mut self, screen: &[[LogicalColor; GB_WIDTH]; GB_HEIGHT]) {
		let texture_creator = self.canvas.texture_creator();
		let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24,  GB_WIDTH as u32, GB_HEIGHT as u32)
			.map_err(|e| e.to_string()).unwrap();

		texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			for y in 0..GB_HEIGHT {
				for x in 0..GB_WIDTH {
					let offset = y * pitch + x * 3;
					let color = self.get_color(screen[y][x]);
					buffer[offset] = color.r as u8;
					buffer[offset + 1] = color.g as u8;
					buffer[offset + 2] = color.b as u8;
				}
			}
		});

		self.canvas.clear();
		self.canvas.copy(&texture, None, None).unwrap();
		self.canvas.present();
	}

	pub fn draw_tilemap(&mut self, tilemap: &[[[LogicalColor; 8]; 8]; 384]) {
		let texture_creator = self.canvas_tilemap.texture_creator();
		let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 128, 192)
			.map_err(|e| e.to_string()).unwrap();

		texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			for tile in 0..384 as usize {
				for row in 0..8 {
					for column in 0..8 {
						let canvas_row = (tile / 16 * 8 + row) as usize;
						let canvas_column = (tile % 16 * 8 + column)  as usize;
						let color = self.get_color(tilemap[tile][row][column]);
						let offset =
							(canvas_row * pitch) + (canvas_column * 3);
						buffer[offset] = color.r as u8;
						buffer[offset + 1] = color.g as u8;
						buffer[offset + 2] = color.b as u8;
					}
				}
			}
		});

		self.canvas_tilemap.clear();
		self.canvas_tilemap.copy(&texture, None, None).unwrap();
		self.canvas_tilemap.present();
	}

	pub fn draw_bg_map(&mut self, bg_map: &[[[LogicalColor; 8]; 8]; 1024]) {
		let texture_creator = self.canvas_bg_map.texture_creator();
		let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
			.map_err(|e| e.to_string()).unwrap();

		texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			for tile in 0..1024 as usize {
				for row in 0..8 {
					for column in 0..8 {
						let canvas_row = (tile / 32 * 8 + row) as usize;
						let canvas_column = (tile % 32 * 8 + column) as usize;
						
						let color = self.get_color(bg_map[tile][row][column]);

						let offset =
							(canvas_row * pitch) + (canvas_column * 3);
						buffer[offset] = color.r as u8;
						buffer[offset + 1] = color.g as u8;
						buffer[offset + 2] = color.b as u8;
					}
				}
			}
		});

		self.canvas_bg_map.clear();
		self.canvas_bg_map.copy(&texture, None, None).unwrap();
		self.canvas_bg_map.present();
	}

	pub fn start_timer(&mut self) {
		self.start = self.timer_subsystem.performance_counter();
	}

	// Sleeps for the current frame so that we can have correct framerate
	pub fn sleep_for_frame(&mut self) {
		
	}
	
	// Prints the framerate
	pub fn print_fps(&mut self) {
		// For the FPS counter
		let end: u64 = self.timer_subsystem.performance_counter();
		let seconds: f64 = (end - self.start) as f64 / self.timer_subsystem.performance_frequency() as f64;
		let current_fps = 1.0 / seconds;
		println!("FPS: {}", current_fps);
	}

	// Returns a RGB color from the Emulators logical color
	fn get_color(&self, logcolor: LogicalColor) -> Color {
		match logcolor {
			LogicalColor::White => self.palette[0],
			LogicalColor::LightGray => self.palette[1],
			LogicalColor::DarkGray => self.palette[2],
			LogicalColor::Black => self.palette[3],
		}
	}
	
	// Returns a simple #XYZABC hex code to 3 integer values
	fn hex_to_rgb(hex: &str) -> Color {
		if hex.len() != 7 {
			panic!("hex_to_rgb_color(): Can't convert hex code: {hex}");
		}

		let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
		let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
		let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
		Color::RGB(r, g, b)
	}
}
