use crate::mmu::MMU;

pub struct Bus {
	pub mmu: MMU,
}

impl Bus {
	pub fn new() -> Self {
		let mmu = MMU::new();
		Bus {
			mmu,
		}
	}

	pub fn initialize(&mut self) {
		self.mmu.initialize();
	}
	
	pub fn tick(&mut self) {
		self.mmu.timer.tick();
		self.mmu.timer.print_state();
		if self.mmu.timer.timer_interrupt == 1 {
			self.mmu.timer.timer_interrupt = 0;
			let mut IF = self.mmu.get_byte(0xFF0F);
			IF = Bus::set_bit(IF, 2, 1);
			self.mmu.set_byte(0xFF0F, IF);
		}
	}

	// Set a bit in a u8
	fn set_bit(value: u8, bit_position: u8, bit_value: u8) -> u8 {
		let new_value = match bit_value {
			0 => value & !(1 << bit_position),
			1 => value | 1 << bit_position,
			_ => panic!("Set bit"),
		};
		new_value
	}
}
