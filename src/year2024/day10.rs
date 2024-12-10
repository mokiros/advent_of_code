use std::{
	collections::{HashMap, HashSet},
	fmt::Display,
	io::{BufRead, Seek},
};

struct Matrix<T> {
	width: u8,
	height: u8,
	data: Vec<T>,
}

impl<T: Copy> Matrix<T> {
	fn new(width: u8, height: u8, data: Vec<T>) -> Self {
		Self {
			data,
			width,
			height,
		}
	}

	fn get(&self, pos: Position) -> Option<T> {
		if pos.x <= self.width && pos.y <= self.height && pos.x > 0 && pos.y > 0 {
			let idx = (pos.y - 1) as usize * self.width as usize + (pos.x - 1) as usize;
			Some(self.data[idx])
		} else {
			None
		}
	}

	fn set(&mut self, pos: Position, value: T) {
		if pos.x <= self.width && pos.y <= self.height && pos.x > 0 && pos.y > 0 {
			let idx = (pos.y - 1) as usize * self.width as usize + (pos.x - 1) as usize;
			self.data[idx as usize] = value;
		} else {
			panic!("Matrix index out of bounds: {} {}", pos.x, pos.y)
		}
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
	x: u8,
	y: u8,
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

fn read_map<R: BufRead>(reader: &mut R) -> (Matrix<u8>, HashSet<Position>) {
	let mut width: u8 = 0;
	let mut height: u8 = 0;
	let mut data: Vec<u8> = Vec::new();
	let mut start_positions: HashSet<Position> = HashSet::new();
	for line in reader.lines() {
		let line = line.unwrap();
		width = 0;
		height += 1;

		for char in line.chars() {
			width += 1;
			let n = char.to_digit(10).unwrap();
			if n == 0 {
				start_positions.insert(Position {
					x: width,
					y: height,
				});
			}
			data.push(n as u8);
		}
	}

	(Matrix::new(width, height, data), start_positions)
}

pub fn solve<R: BufRead + Seek>(mut reader: R) -> (i64, i64) {
	let (map, positions) = read_map(&mut reader);

	let mut p1 = 0;
	let mut p2 = 0;
	for pos in positions {
		let mut endpoints = Vec::new();
		let mut next_endpoints = Vec::new();

		endpoints.push(pos);

		for i in 1..=9 {
			for pos in endpoints.iter() {
				for dir in [[2, 1], [1, 2], [0, 1], [1, 0]] {
					let next_pos = Position {
						x: pos.x + dir[0] as u8 - 1,
						y: pos.y + dir[1] as u8 - 1,
					};
					if map.get(next_pos) == Some(i) {
						next_endpoints.push(next_pos);
					}
				}
			}

			let temp = endpoints;
			endpoints = next_endpoints;
			next_endpoints = temp;
			next_endpoints.clear();
		}

		if endpoints.len() > 0 {
			p2 += endpoints.len() as i64;
			p1 += HashSet::<Position>::from_iter(endpoints).len();
		}
	}

	(p1 as i64, p2 as i64)
}
