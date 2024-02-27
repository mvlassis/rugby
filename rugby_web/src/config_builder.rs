use config::Config;
use crate::app::Palette;

// Get all palettes from config.toml
pub fn get_all_palettes() -> Vec<Palette> {
	let config = Config::builder()
		.add_source(config::File::with_name("config.toml"))
		.build()
		.unwrap();

	let palette_names = config.get_array("frontend.palettes_available").unwrap();
	let palette_names: Vec<String> = palette_names.into_iter().filter_map(|value| value.into_string().ok()).collect();
	let mut all_palettes: Vec<Palette> = Vec::new();
		
	for name in &palette_names {
		let new_palette = get_palette_from_config(&config, name);
		all_palettes.push(new_palette);
	}

	// Get the selected palette, then bring it to the front of the array
	let selected_palette_name = config.get_string("frontend.default_palette").unwrap();
	if let Some(index) = palette_names.iter().position(|x| *x == selected_palette_name) {
		let (front, back) = all_palettes.split_at_mut(index);
		all_palettes = back.iter().chain(front.iter()).cloned().collect();
	}
	all_palettes
}


// Return a palette from the [frontend] section in a config builder
fn get_palette_from_config(config: &Config, name: &str) -> Palette {
	let palette_name = "frontend.".to_string() + name;
	let palette_colors = config.get_array(&palette_name).unwrap();
	let palette_colors: Vec<String> = palette_colors.into_iter().filter_map(|value| value.into_string().ok()).collect();

	let color0 = hex_to_rgb(&palette_colors[0]).unwrap_or((0, 0, 0));
	let color1 = hex_to_rgb(&palette_colors[1]).unwrap_or((211, 211, 211));
	let color2 = hex_to_rgb(&palette_colors[2]).unwrap_or((90, 90, 90));
	let color3 = hex_to_rgb(&palette_colors[3]).unwrap_or((0, 0, 0));

	Palette {
		name: name.to_string().replace("_", " "),
		colors: [color0, color1, color2, color3]
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
