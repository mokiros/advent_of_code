use super::position::Position;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[allow(unused)]
impl Direction {
	pub const fn next(self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}

	pub const fn previous(self) -> Self {
		match self {
			Self::Up => Self::Left,
			Self::Right => Self::Up,
			Self::Down => Self::Right,
			Self::Left => Self::Down,
		}
	}

	pub const fn update_position(self, pos: &Position) -> Position {
		match self {
			Self::Up => Position {
				x: pos.x,
				y: pos.y - 1,
			},
			Self::Right => Position {
				x: pos.x + 1,
				y: pos.y,
			},
			Self::Down => Position {
				x: pos.x,
				y: pos.y + 1,
			},
			Self::Left => Position {
				x: pos.x - 1,
				y: pos.y,
			},
		}
	}

	pub const fn opposite(self) -> Self {
		match self {
			Self::Up => Self::Down,
			Self::Right => Self::Left,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
		}
	}

	pub const fn orthogonal(self) -> (Self, Self) {
		match self {
			Self::Up | Self::Down => (Self::Left, Self::Right),
			Self::Left | Self::Right => (Self::Up, Self::Down),
		}
	}

	pub fn distance(self, other_direction: Self) -> u8 {
		if self == other_direction {
			0
		} else if self.opposite() == other_direction {
			2
		} else {
			1
		}
	}

	pub fn all() -> Vec<Self> {
		vec![Self::Up, Self::Right, Self::Down, Self::Left]
	}
}
