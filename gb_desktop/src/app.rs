use eframe::egui;
use egui::{Color32, Frame, InputState, Key, Vec2};
use rfd::FileDialog;

use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::TimerSubsystem;
use std::env;
use std::time::Duration;
use std::path::PathBuf;

use gb_core::color::Color as LogicalColor;
use gb_core::emulator::Emulator;
use gb_core::input::Input;
use gb_core::input::EmulatorInput;
use crate::config_builder::get_all_palettes;

const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;

#[derive(Clone)]
pub struct Palette {
	pub colors: [(u8, u8, u8); 4],
}

pub fn run_app() {
	let sdl_context = sdl2::init().unwrap();
	let timer_subsystem = sdl_context.timer().unwrap();
	
	// Setup the audio
	let audio_subsystem = sdl_context.audio().expect("Failed to initialize audio");
	let desired_spec = AudioSpecDesired {
		freq: Some(44100),
		channels: Some(2),
		samples: Some(1024),
	};
	let audio_queue: AudioQueue<f32> = audio_subsystem.open_queue(None, &desired_spec)
		.expect("Failed to create audio queue");
	audio_queue.resume();
	let callback = Box::new(move |buffer: &[f32]| {
		while audio_queue.size() > 1024 * 4 * 2 {
			std::thread::sleep(Duration::from_millis(1));
		}
		let _ = audio_queue.queue_audio(buffer);
	});

	let palettes = get_all_palettes();
	
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_active(true)
			.with_inner_size([(320) as f32, (312) as f32])
			.with_resizable(false),
		vsync: false, 
		centered: true,
		..Default::default()
	};
	
	let _ = eframe::run_native("Rugby", native_options,
							   Box::new(|cc| {
								   cc.egui_ctx.set_pixels_per_point(1.0);
								   
								   let args: Vec<String> = env::args().collect();
								   let first_arg = &args[1];

								   let file_path = PathBuf::from(first_arg);
								   
								   Box::new(EguiApp::new(cc, palettes, timer_subsystem, file_path, callback))
							   })
	);
}

pub struct EguiApp {
	gb: Emulator,
	palettes: Vec<Palette>,
	current_palette_index: usize,

	toggle_mute: bool,
	show_palette_window: bool,
	
	timer_subsystem: TimerSubsystem,
	start: u64,
	end: u64,
	first_frame: bool,
}

impl EguiApp {
	pub fn new(_cc: &eframe::CreationContext<'_>, palettes: Vec<Palette>, timer: TimerSubsystem, path_buf: PathBuf, callback: Box<dyn Fn(&[f32])>) -> Self {
		let gb = Emulator::new(path_buf, callback);
		
		let start = timer.performance_counter();
		let end = timer.performance_counter();
		
		EguiApp {
			gb,
			palettes,
			current_palette_index: 0,

			toggle_mute: false,
			show_palette_window: false,
			timer_subsystem: timer,
			start,
			end,
			first_frame: true,
		}
    }

	fn handle_input(&mut self, input_state: &InputState) -> (Input, EmulatorInput) {
		let mut input = Input::new();
		let mut emulator_input = EmulatorInput::new();

		if input_state.key_pressed(Key::Escape) {
			emulator_input.exit = true;
		}
		if input_state.key_down(Key::ArrowUp) {
			input.up = true;
		}
		if input_state.key_down(Key::ArrowDown) {
			input.down = true;
		}
		if input_state.key_down(Key::ArrowLeft) {
			input.left = true;
		}
		if input_state.key_down(Key::ArrowRight) {
			input.right = true;
		}
		if input_state.key_down(Key::A) {
			input.a = true;
		}
		if input_state.key_down(Key::S) {
			input.b = true;
		}
		if input_state.key_down(Key::Z) {
			input.start = true;
		}
		if input_state.key_down(Key::X) {
			input.select = true;
		}

		if self.toggle_mute {
			emulator_input.toggle_mute = true;
			self.toggle_mute = false;
		}
		
		(input, emulator_input)
	}

	pub fn start_timer(&mut self) {
		self.start = self.timer_subsystem.performance_counter();
	}
	
	#[allow(dead_code)]
	// Prints the framerate
	pub fn print_fps(&mut self) {
		self.end = self.timer_subsystem.performance_counter();
		let seconds: f64 = (self.end - self.start) as f64 / self.timer_subsystem.performance_frequency() as f64;
		println!("Seconds: {}", seconds);
		let current_fps = 1.0 / seconds;
		println!("FPS: {}", current_fps);
	}

	// Returns a RGB color from the Emulators logical color
	fn get_color(&self, logcolor: &LogicalColor) -> (u8, u8, u8) {
		let i = self.current_palette_index;
		match logcolor {
			LogicalColor::White => self.palettes[i].colors[0],
			LogicalColor::LightGray => self.palettes[i].colors[1],
			LogicalColor::DarkGray => self.palettes[i].colors[2],
			LogicalColor::Black => self.palettes[i].colors[3],
		}
	}

}

impl eframe::App for EguiApp {
	
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.start_timer();
		let mut input = Input::new();
		let mut emulator_input = EmulatorInput::new();

		ctx.input(|i| {
			(input, emulator_input) = self.handle_input(i);
		});
		
		let screen = self.gb.run(input, Some(emulator_input)).clone();

		let mut buffer: Vec<u8> = Vec::with_capacity(GB_WIDTH * GB_HEIGHT * 4);
		for y in 0..GB_HEIGHT {
			for x in 0..GB_WIDTH {
				let color = screen[y][x];
				let (r, g, b) = self.get_color(&color);
				buffer.push(r);
				buffer.push(g);
				buffer.push(b);
			}
		}

		// Menu bar
		egui::TopBottomPanel::top("Menu bar").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Open").clicked()  {
						let file = FileDialog::new()
							.add_filter("Game Boy", &["gb", "gbc", "bin"])
							.pick_file();
						if let Some(path) = file {
							self.gb.load(path);
						}
						ui.close_menu();
					}
				});
				if ui.button("Palette").clicked() {
					self.show_palette_window = !self.show_palette_window;
				}
				if ui.button("Mute").clicked() {
					self.toggle_mute = true;
				}
			});
		});
		
		egui::CentralPanel::default().frame(Frame::none()).show(ctx, |ui| {
			if self.first_frame {
				self.first_frame = false;
				// ctx.set_pixels_per_point(1.0);
			}

			let size = [GB_WIDTH as _, GB_HEIGHT as _];
			let image = egui::ColorImage::from_rgb(size, &buffer);
		   
			let texture_handle = ui.ctx().load_texture("Game screen", image, egui::TextureOptions::NEAREST);
			let scaled_size = Vec2::new(GB_WIDTH as f32 * 2.0, GB_HEIGHT as f32 * 2.0);
			ui.image((texture_handle.id(), scaled_size));
		});

		// Palette window
		egui::Window::new("Palette")
			.open(&mut self.show_palette_window)
			.show(ctx, |ui| {
			let current_palette = &self.palettes[self.current_palette_index];
			ui.horizontal(|ui| {

				if ui.button("<").clicked() {
					self.current_palette_index = match self.current_palette_index {
						0 => self.palettes.len() - 1,
						_ => self.current_palette_index - 1,
					};
				}
				for &color in &current_palette.colors {
					let (r, g, b) = color;
					let color32 = Color32::from_rgb(r, g, b);
					let rect = ui.allocate_space(egui::Vec2::splat(35.0));
					ui.painter().rect_filled(rect.1, 0.0, color32);
				}
                if ui.button(">").clicked() {
					self.current_palette_index = (self.current_palette_index + 1) % self.palettes.len();
                }
			})
		});
		
		// self.print_fps();
		ctx.request_repaint();
   }
}
