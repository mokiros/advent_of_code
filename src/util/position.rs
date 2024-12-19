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

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
