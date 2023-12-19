use serde::{Serialize, Deserialize};

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum Color {
	White,
	LightGray,
	DarkGray,
	Black,
}
