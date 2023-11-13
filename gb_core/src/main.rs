use gb_core::GameBoy;

fn main() {
	let mut gb = GameBoy::new();
	loop {
		gb.run();
	}
}
