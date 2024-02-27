use eframe::egui;
use eframe::Storage;
use egui::{Color32, Frame, InputState, Key, Vec2, ViewportCommand};
use rfd::AsyncFileDialog;
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{Receiver, Sender};
use std::path::PathBuf;

use rugby_core::color::Color as OutputColor;
use rugby_core::color::LogicalColor;
use rugby_core::emulator::Emulator;
use rugby_core::input::Input;
use rugby_core::input::EmulatorInput;

const GB_WIDTH: usize = 160;
const GB_HEIGHT: usize = 144;
const MENUBAR_HEIGHT: f32 = 20.0;

#[derive(Clone, PartialEq)]
pub struct Palette {
	pub name: String,
	pub colors: [(u8, u8, u8); 4],
}

pub fn run_app() {
	eframe::WebLogger::init(log::LevelFilter::Debug).ok();

	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();
	sink.play();

	let callback = Box::new(move |buffer: &[f32]| {
		sink.append(SamplesBuffer::new(2, 44100, buffer));
	});

	let palette = Palette {
		name: "Ice Cream GB".to_string(),
		colors: [hex_to_rgb("#fff6d3").unwrap(),
				 hex_to_rgb("#f9a875").unwrap(),
				 hex_to_rgb("#eb6b6f").unwrap(),
				 hex_to_rgb("#7c3f58").unwrap(),]
	};
	
	let palettes = vec![palette];
	
	let scale = Box::new(5.0); // Default scale
	
	let web_options = eframe::WebOptions::default();
	wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
					let first_arg: Option<String> = env::args().nth(1).clone();
					Box::new(EguiApp::new(cc, palettes, scale, first_arg, callback))
				})
            )
            .await
            .expect("failed to start eframe");
    });
	
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
	recent_roms: Vec<PathBuf>,

	async_channels: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
}

impl EguiApp {
	pub fn new(cc: &eframe::CreationContext<'_>, palettes: Vec<Palette>, scale: Box<f32>,
			   file_arg: Option<String>, callback: Box<dyn Fn(&[f32])>) -> Self {
		let gb = match file_arg {
			Some(s) => {
				let path_buf = PathBuf::from(s);
				let mut rom = File::open(path_buf.clone()).expect("Unable to open file {path}");
				let mut data_buffer = Vec::new();
				rom.read_to_end(&mut data_buffer).unwrap();
				Emulator::new(Some(data_buffer), Some(path_buf), callback)
			},
			None => Emulator::new(None, None, callback),
		};
		
		let recent_roms = eframe::get_value(cc.storage.unwrap(), "recent_roms").unwrap_or_default();
		let palette_index = 0;
		
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
			recent_roms,
			async_channels: std::sync::mpsc::channel(),
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
			// Disable rewind in web
			emulator_input.rewind = false;
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

	// Returns a RGB color from the emulator's logical color
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
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut input = Input::new();
		let mut emulator_input = EmulatorInput::new();

		loop {
            match self.async_channels.1.try_recv() {
                Ok(data_buffer) =>  {
                    self.gb.load(Some(data_buffer), None);
                },
                Err(_) => break,
            }
        }
		
		ctx.input(|i| {
			(input, emulator_input) = self.handle_input(i);
			if i.viewport().close_requested() {
				// Save before we exit the program
				self.gb.save();
			}
		});

		// Run the emulator for a frame
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
						let sender = self.async_channels.0.clone();
						let future = async move {
							let file = AsyncFileDialog::new()
								.add_filter("Game Boy", &["gb", "gbc", "sgb", "bin"])
								.pick_file()
								.await;
							let data = file.unwrap().read().await;
							sender.send(data).ok();
						};

						wasm_bindgen_futures::spawn_local(future);
						
						ui.close_menu();
					}
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
			});
		});
		
		egui::CentralPanel::default().frame(Frame::none()).show(ctx, |ui| {
			let size = [GB_WIDTH as _, GB_HEIGHT as _];
			let image = egui::ColorImage::from_rgb(size, &buffer);
		   
			let texture_handle = ui.ctx().load_texture("Game screen", image, egui::TextureOptions::NEAREST);
			let scaled_size = Vec2::new((GB_WIDTH as f32 * self.scale) / ctx.pixels_per_point(),
										(GB_HEIGHT as f32 * self.scale) / ctx.pixels_per_point());
			ui.centered_and_justified(|ui| {
				ui.image((texture_handle.id(), scaled_size));
			});
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
		ctx.request_repaint();
	}

	fn save(&mut self, storage: &mut dyn Storage) {
		eframe::set_value(storage, "recent_roms", &self.recent_roms);
		eframe::set_value(storage, "palette", &self.palettes[self.palette_index].name);
	}

}

// Returns a simple #XYZABC hex code to 3 integer values
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
	if hex.len() != 7 {
		return None;
	}

	let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
	let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
	let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
	Some((r, g, b))
}
