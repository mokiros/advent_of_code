use super::position::Position;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	pub fn next(&self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}

	pub fn update_position(&self, pos: Position) -> Position {
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

	pub fn opposite(&self) -> Self {
		match self {
			Self::Up => Self::Down,
			Self::Right => Self::Left,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
		}
	}
}
