pub struct Timer {
	internal_timer: u16,
	previous_and: u8,
	tima_overflow: bool,
	pub div: u8,
	pub tima: u8,
	pub tma: u8,
	pub tac: u8,
	pub timer_interrupt: u8,
}
impl Timer {
	pub fn new() -> Self {
		Timer {
			internal_timer: 0,
			previous_and: 0,
			tima_overflow: false,
			div: 0,
			tima: 0,
			tma: 0,
			tac: 0,
			timer_interrupt: 0,
		}
	}

	pub fn initialize(&mut self) {
		self.internal_timer = 0xAB00;
		self.div = 0xAB;
		self.tima = 0;
		self.tma = 0;
		self.tac = 0xF8;
	}

	// Update all timers. This is called every M-Cycle
	pub fn tick(&mut self) {
		self.internal_timer = self.internal_timer.wrapping_add(4);
		self.div = (self.internal_timer >> 8) as u8;

		if self.tima_overflow && self.tima == 0 {
			self.tima_overflow = false;
			self.tima = self.tma;
			self.timer_interrupt = 1;
		}
		
		let timer_enable = (self.tac >> 2) & 0x1;
		let select_bit = match self.tac & 0x03 {
			0b00 => Timer::get_bit(self.internal_timer, 9),
			0b01 => Timer::get_bit(self.internal_timer, 3),
			0b10 => Timer::get_bit(self.internal_timer, 5),
			0b11 => Timer::get_bit(self.internal_timer, 7),
			_ => panic!("Timer.tick()"),
		};
		let and_result = select_bit & timer_enable;
		if and_result == 0 && self.previous_and == 1 {
			let (new_value, overflow) = self.tima.overflowing_add(1);
			self.tima = new_value;
			if overflow {
				self.tima_overflow = true;
			}
		}
		self.previous_and = and_result;
	}

	// Resets the internal timer to 0
	pub fn reset_timer(&mut self) {
		self.internal_timer = 0;
		self.div = 0;
	}

	fn get_bit(value: u16, bit_position: u8) -> u8 {
		let bit = (value >> bit_position) & 0x1;
		bit as u8
	}
}
