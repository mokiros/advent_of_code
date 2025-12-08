use std::{
	collections::HashSet,
	io::{BufRead, Seek},
};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

fn read_map<R: BufRead>(reader: &mut R) -> (Matrix<u8>, HashSet<Position>) {
	let mut width = 0;
	let mut height = 0;
	let mut data: Vec<u8> = Vec::new();
	let mut start_positions: HashSet<Position> = HashSet::new();
	for line in reader.lines() {
		let line = line.unwrap();
		width = 0;

		for char in line.chars() {
			let n = char.to_digit(10).unwrap();
			if n == 0 {
				start_positions.insert(Position {
					x: width,
					y: height,
				});
			}
			data.push(u8::try_from(n).unwrap());
			width += 1;
		}
		height += 1;
	}

	(
		Matrix::new(
			u8::try_from(width).unwrap(),
			u8::try_from(height).unwrap(),
			data,
		),
		start_positions,
	)
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let (map, positions) = read_map(&mut reader);

	let mut p1 = 0;
	let mut p2 = 0;
	for pos in positions {
		let mut endpoints = Vec::new();
		let mut next_endpoints = Vec::new();

		endpoints.push(pos);

		for i in 1..=9 {
			for pos in &endpoints {
				for dir in Direction::all() {
					let next_pos = dir.update_position(pos);
					if map.get(next_pos.x, next_pos.y) == Some(i) {
						next_endpoints.push(next_pos);
					}
				}
			}

			std::mem::swap(&mut endpoints, &mut next_endpoints);
			next_endpoints.clear();
		}

		if !endpoints.is_empty() {
			p2 += i64::try_from(endpoints.len()).unwrap();
			p1 += HashSet::<Position>::from_iter(endpoints).len();
		}
	}

	(p1.to_string(), p2.to_string())
}
