use eframe::egui;
use eframe::Storage;
use egui::{Color32, Frame, InputState, Key, Vec2, ViewportCommand};
use rfd::FileDialog;
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::TimerSubsystem;
use std::env;
use std::time::Duration;
use std::path::PathBuf;
use winit::event_loop::EventLoop;

use rugby_core::color::Color as OutputColor;
use rugby_core::color::LogicalColor;
use rugby_core::emulator::Emulator;
use rugby_core::input::Input;
use rugby_core::input::EmulatorInput;
use crate::config_builder::get_all_palettes;

const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;
const MENUBAR_HEIGHT: f32 = 20.0;
const RECENT_ROMS_LENGTH: usize = 5;

#[derive(Clone, PartialEq)]
pub struct Palette {
	pub name: String,
	pub colors: [(u8, u8, u8); 4],
}

pub fn run_app() {
	// Quickly get the display's DPI
	let event_loop = EventLoop::new();
	let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .expect("Failed to create window");
	let dpi_factor = window.scale_factor() as f32;
	drop(window);
	
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
	let scale = Box::new(4.0); // Default scale
	
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_active(true)
			// 20 is a magic number, the menu bar's height
			.with_inner_size([(GB_WIDTH as f32 * *scale) / dpi_factor,
							  ((GB_HEIGHT as f32 * *scale) / dpi_factor) + (MENUBAR_HEIGHT + 1.0 * *scale) as f32])
			.with_resizable(false),
		vsync: false, 
		centered: true,
		persist_window: false,
		..Default::default()
	};
	
	let _ = eframe::run_native("Rugby", native_options,
							   Box::new(|cc| {
								   let ppp = cc.egui_ctx.native_pixels_per_point();
								   if let Some(value) = ppp {
									   cc.egui_ctx.send_viewport_cmd(ViewportCommand::InnerSize(
										   Vec2::new((GB_WIDTH as f32 * *scale) / value,
													 (GB_HEIGHT as f32 * *scale) / value + (MENUBAR_HEIGHT + 1.5 * *scale))));
								   }
								   let first_arg: Option<String> = env::args().nth(1).clone();
								   Box::new(EguiApp::new(cc, palettes, scale, timer_subsystem, first_arg, callback))
							   })
	);
}

pub struct EguiApp {
	gb: Emulator,
	palettes: Vec<Palette>,
	palette_index: usize,

	scale: f32,
	emulator_playing: bool,
	exit_program: bool,
	active_layers: [bool; 3],
	toggle_layers: [bool; 3],
	toggle_mute: bool,
	audio_on: bool,
	active_channels: [bool; 4],
	toggle_channels: [bool; 4],
	show_palette_window: bool,
	select_save_state: (bool, usize),
	select_load_state: (bool, usize),
	timer_subsystem: TimerSubsystem,
	start: u64,
	end: u64,
	recent_roms: Vec<PathBuf>,
}

impl EguiApp {
	pub fn new(cc: &eframe::CreationContext<'_>, palettes: Vec<Palette>, scale: Box<f32>, timer: TimerSubsystem,
			   file_arg: Option<String>, callback: Box<dyn Fn(&[f32])>) -> Self {
		let gb = match file_arg {
			Some(s) => {
				let path_buf = PathBuf::from(s);
				Emulator::new(Some(path_buf), callback)
			},
			None => Emulator::new(None, callback),
		};
		
		let start = timer.performance_counter();
		let end = timer.performance_counter();
		let recent_roms = eframe::get_value(cc.storage.unwrap(), "recent_roms").unwrap_or_default();
		let mut palette_index = 0;
		let palette: Option<String> = eframe::get_value(cc.storage.unwrap(), "palette");
		if let Some(palette_name) = palette {
			let index = palettes.iter().position(|p| p.name == palette_name);
			if let Some(index) = index {
				palette_index = index;
			}
		}
		
		EguiApp {
			gb,
			palettes,
			palette_index,

			scale: *scale,
			emulator_playing: true,
			exit_program: false,
			active_layers: [true; 3],
			toggle_layers: [false; 3],
			toggle_mute: false,
			audio_on: true,
			active_channels: [true; 4],
			toggle_channels: [false; 4],
			show_palette_window: false,
			select_save_state: (false, 0),
			select_load_state: (false, 0),
			timer_subsystem: timer,
			start,
			end,
			recent_roms,
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
		if input_state.key_pressed(Key::O) {
			emulator_input.save_state = true;
		}
		if input_state.key_pressed(Key::P) {
			emulator_input.load_state = true;
		}
		if input_state.key_down(Key::R) {
			emulator_input.rewind = true;
		}

		if self.toggle_mute {
			emulator_input.toggle_mute = true;
			self.toggle_mute = false;
		}
		if self.exit_program {
			emulator_input.exit = true;
			self.exit_program = false;
		}
		for i in 0..=2 {
			if self.toggle_layers[i] {
				emulator_input.toggle_layer[i] = true;
				self.toggle_layers[i] = false;
			}
		}
		for i in 0..=3 {
			if self.toggle_channels[i] {
				emulator_input.toggle_channel[i] = true;
				self.toggle_channels[i] = false;
			}
		}
		if self.select_save_state.0 {
			emulator_input.select_save_state = (true, self.select_save_state.1);
			self.select_save_state = (false, 0);
		}
		if self.select_load_state.0 {
			emulator_input.select_load_state = (true, self.select_load_state.1);
			self.select_load_state = (false, 0);
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
		// println!("Seconds: {}", seconds);
		let current_fps = 1.0 / seconds;
		println!("FPS: {}", current_fps);
	}

	// Returns a RGB color from the Emulators logical color
	fn get_color(&self, color: &OutputColor) -> (u8, u8, u8) {
		let i = self.palette_index;
		match color {
			OutputColor::Logical(logical) => match logical {
				LogicalColor::White => self.palettes[i].colors[0],
				LogicalColor::LightGray => self.palettes[i].colors[1],
				LogicalColor::DarkGray => self.palettes[i].colors[2],
				LogicalColor::Black => self.palettes[i].colors[3],
			},
			OutputColor::RGB(rgb) => {
				let r_5bit = (rgb & 0x1F) as u8;
				let r_8bit = (r_5bit << 3) | (r_5bit >> 2);
				
				let g_5bit = ((rgb & 0x3E0) >> 5) as u8;
				let g_8bit = (g_5bit << 3) | (g_5bit >> 2);
				
				let b_5bit = ((rgb & 0x7C00) >> 10) as u8;
				let b_8bit = (b_5bit << 3) | (b_5bit >> 2);
				(r_8bit, g_8bit, b_8bit)
			}
		}
	}
}

impl eframe::App for EguiApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		self.start_timer();
		let mut input = Input::new();
		let mut emulator_input = EmulatorInput::new();

		ctx.input(|i| {
			(input, emulator_input) = self.handle_input(i);
			if i.viewport().close_requested() {
				self.gb.save();
			}
		});

		let screen = match self.emulator_playing {
			true => self.gb.run(input, Some(emulator_input)).clone(),
			false => self.gb.get_screen().clone(),
		};


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
							.add_filter("Game Boy", &["gb", "gbc", "sgb", "bin"])
							.pick_file();
						if let Some(path) = file {
							self.gb.load(path.clone());
							
							if !self.recent_roms.contains(&path) {
								if self.recent_roms.len() > RECENT_ROMS_LENGTH {
									self.recent_roms.remove(0);
								}
								self.recent_roms.push(path.clone());
							}
							// Update the storage
							if let Some(storage) = frame.storage_mut() {
                                eframe::set_value(storage, "recent_roms", &self.recent_roms);
                                storage.flush();
                            }
						}
						ui.close_menu();
					}
					ui.menu_button("Recent Files", |ui| {
						for rom_path in self.recent_roms.clone().iter().rev() {
							if ui.button(rom_path.file_name().unwrap().to_str().unwrap()).clicked() {
								self.gb.load(rom_path.to_path_buf());
							}
						}
					});
				});
				// Options
				ui.menu_button("Options", |ui| {
					ui.checkbox(&mut self.emulator_playing, "Pause/Resume");
				});
				// Video
				ui.menu_button("Video", |ui| {
					ui.menu_button("Scaling", |ui| {
						for i in 1..=5 {
							if ui.radio_value(&mut self.scale,
											  i as f32, format!("{}x", i)).clicked() {
								self.scale = i as f32;
								ctx.send_viewport_cmd(ViewportCommand::InnerSize(
									Vec2::new((GB_WIDTH as f32 * self.scale) / ctx.pixels_per_point()
											  , (GB_HEIGHT as f32 * self.scale) / ctx.pixels_per_point()
											  + (MENUBAR_HEIGHT + 1.5 * self.scale))));
							}
						}
					});
					ui.menu_button("Palettes", |ui| {
						for i in 0..self.palettes.len() {
							if ui.radio_value(&mut self.palette_index,
											  i, &self.palettes[i].name).clicked() {
								self.palette_index = i;
							}
						}
					});
					if ui.button("Palette Picker").clicked() {
						self.show_palette_window = !self.show_palette_window;
					}
					ui.menu_button("Video Layers", |ui| {
						for i in 0..=2 {
							if ui.checkbox(&mut self.active_layers[i],
										   format!("Layer {}", i)).clicked() {
								self.toggle_layers[i] = true;
							}
						}
					});
				});
				// Audio
				ui.menu_button("Audio", |ui| {
					if ui.checkbox(&mut self.audio_on, "Toggle Audio").clicked() {
						self.toggle_mute = true;
					}
					ui.menu_button("Audio Channels", |ui| {
						for i in 0..=3 {
							if ui.checkbox(&mut self.active_channels[i],
										   format!("Channel {}", i)).clicked() {
								self.toggle_channels[i] = true;
							}
						}
					});
				});
				// Save
				ui.menu_button("Save", |ui| {
					ui.menu_button("Save State", |ui| {
						for i in 0..=3 {
							let save_status = self.gb.select_save_states[i] != "";
							if ui.selectable_label(save_status, format!("Save to State Slot {}", i)).clicked() {
								self.select_save_state = (true, i);
							}

						}
					});
					ui.menu_button("Load State", |ui| {
						for i in 0..=3 {
							let save_status = self.gb.select_save_states[i] != "";
							if ui.selectable_label(save_status, format!("Load from State Slot {}", i)).clicked() {
								self.select_load_state = (true, i);
							}
						}
					});
				});
				// Exit
				if ui.button("Exit").clicked() {
					self.exit_program = true;
				}
			});
		});
		
		egui::CentralPanel::default().frame(Frame::none()).show(ctx, |ui| {
			let size = [GB_WIDTH as _, GB_HEIGHT as _];
			let image = egui::ColorImage::from_rgb(size, &buffer);
		   
			let texture_handle = ui.ctx().load_texture("Game screen", image, egui::TextureOptions::NEAREST);
			let scaled_size = Vec2::new((GB_WIDTH as f32 * self.scale) / ctx.pixels_per_point(),
										(GB_HEIGHT as f32 * self.scale) / ctx.pixels_per_point());
			ui.image((texture_handle.id(), scaled_size));
		});

		// Palette window
		egui::Window::new("Palette Picker")
			.open(&mut self.show_palette_window)
			.show(ctx, |ui| {
				let current_palette = &self.palettes[self.palette_index];
				ui.label(&current_palette.name);
				ui.horizontal(|ui| {
					if ui.button("<").clicked() {
						self.palette_index = match self.palette_index {
							0 => self.palettes.len() - 1,
							_ => self.palette_index - 1,
						};
					}
					for &color in &current_palette.colors {
						let (r, g, b) = color;
						let color32 = Color32::from_rgb(r, g, b);
						let rect = ui.allocate_space(Vec2::splat(20.0));
						ui.painter().rect_filled(rect.1, 0.0, color32);
					}
					if ui.button(">").clicked() {
						self.palette_index = (self.palette_index + 1) % self.palettes.len();
					}
				})
			});
		// self.print_fps();
		ctx.request_repaint();
	}

	fn save(&mut self, storage: &mut dyn Storage) {
		eframe::set_value(storage, "recent_roms", &self.recent_roms);
		eframe::set_value(storage, "palette", &self.palettes[self.palette_index].name);
	}

}
