use gb_core::emulator::Emulator;

use std::env;
// use std::thread;
// use std::time::Duration;

fn main() {
	let args: Vec<String> = env::args().collect();
	let first_arg = &args[1];
	// println!("{}", std::env::current_dir().unwrap().display());
	let mut gb = Emulator::new();
	gb.load(first_arg);
	loop {
		gb.run();
		// thread::sleep(Duration::from_millis(1));
	}
}
