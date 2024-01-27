use serde::{Serialize, Deserialize};

#[derive(Clone, Copy)]
pub enum Color {
	Logical(LogicalColor),
	RGB(u16),
}

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum LogicalColor {
	White,
	LightGray,
	DarkGray,
	Black,
}
