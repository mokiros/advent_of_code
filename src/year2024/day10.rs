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
			data.push(n as u8);
			width += 1;
		}
		height += 1;
	}

	(
		Matrix::new(width as u8, height as u8, data),
		start_positions,
	)
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
				for dir in Direction::all() {
					let next_pos = dir.update_position(pos);
					if map.get(next_pos.x, next_pos.y) == Some(i) {
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
