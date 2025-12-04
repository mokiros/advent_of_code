use std::io::{BufRead, Seek};

use crate::util::position::Position;

struct Map<T: Copy> {
	width: isize,
	height: isize,
	data: Vec<T>,
}

impl<T: Copy> Map<T> {
	pub const fn new(width: isize, height: isize, data: Vec<T>) -> Self {
		Self {
			width,
			height,
			data,
		}
	}

	pub fn read_map<R: BufRead>(reader: R, char_fn: fn(char: u8) -> T) -> Self {
		let mut data: Vec<T> = Vec::new();
		let mut width = 0;
		let mut height = 0;

		let mut bytes = reader.bytes();

		while let Some(Ok(byte)) = bytes.next() {
			if byte == b'\n' {
				height += 1;
				continue;
			}
			if height == 0 {
				width += 1;
			}
			data.push(char_fn(byte));
		}

		Self::new(width, height, data)
	}

	const fn get_idx(&self, x: isize, y: isize) -> Option<usize> {
		let w = self.width;
		let h = self.height;
		if x < w && y < h && x >= 0 && y >= 0 {
			Some((y * w + x).cast_unsigned())
		} else {
			None
		}
	}

	pub fn get(&self, x: isize, y: isize) -> Option<T> {
		self.get_idx(x, y).map(|idx| self.data[idx])
	}

	pub fn set(&mut self, x: isize, y: isize, value: T) {
		if let Some(idx) = self.get_idx(x, y) {
			self.data[idx] = value;
		} else {
			panic!("Matrix index out of bounds: {x} {y}")
		}
	}
}

#[derive(Clone, Copy)]
enum Tile {
	Empty,
	Paper,
}

#[allow(clippy::fallible_impl_from)]
impl From<u8> for Tile {
	fn from(value: u8) -> Self {
		match value {
			b'.' => Self::Empty,
			b'@' => Self::Paper,
			tile => panic!("Invalid tile: {tile}"),
		}
	}
}

const DIRECTIONS: [Position; 8] = [
	Position { x: -1, y: -1 },
	Position { x: 0, y: -1 },
	Position { x: 1, y: -1 },
	Position { x: -1, y: 0 },
	// Position{x: 0, y: 0},
	Position { x: 1, y: 0 },
	Position { x: -1, y: 1 },
	Position { x: 0, y: 1 },
	Position { x: 1, y: 1 },
];

fn do_pass(map: &mut Map<Tile>, clear_paper: bool) -> u64 {
	let mut removed = 0;

	for y in 0..map.height {
		for x in 0..map.width {
			if matches!(map.get(x, y), Some(Tile::Paper)) {
				let pos = Position { x, y };
				let mut paper_count = 0;

				for direction in DIRECTIONS {
					let neighbour_position = pos + direction;
					if matches!(
						map.get(neighbour_position.x, neighbour_position.y),
						Some(Tile::Paper)
					) {
						paper_count += 1;
						if paper_count >= 4 {
							break;
						}
					}
				}

				if paper_count <= 3 {
					if clear_paper {
						map.set(x, y, Tile::Empty);
					}
					removed += 1;
				}
			}
		}
	}

	removed
}

pub fn solve<R: BufRead + Seek>(reader: R) -> (String, String) {
	let mut part2: u64 = 0;

	let mut map = Map::read_map(reader, Tile::from);

	let part1: u64 = do_pass(&mut map, false);

	let mut removed;

	loop {
		removed = do_pass(&mut map, true);
		part2 += removed;
		if removed == 0 {
			break;
		}
	}

	(part1.to_string(), part2.to_string())
}
