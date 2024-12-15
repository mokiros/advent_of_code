#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
	pub x: isize,
	pub y: isize,
}

impl Position {
	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y }
	}
}
