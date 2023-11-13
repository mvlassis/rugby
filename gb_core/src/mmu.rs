const MEMORY_SIZE: usize = 4096;
const ROM_START_ADDRESS: usize = 0x100;

// Gameboy does not actually have an MMU, don't tell the Nintendo ninjas
pub struct MMU {
	memory: [u8; MEMORY_SIZE],
}

impl MMU {
	pub fn new() -> Self {
		MMU {
			memory: [0; MEMORY_SIZE],
		}
	}

	pub fn load(&mut self, data_buffer: &Vec<u8>) {
		let end = ROM_START_ADDRESS + data_buffer.len();
		self.memory[ROM_START_ADDRESS..end].copy_from_slice(data_buffer);
	}
}
