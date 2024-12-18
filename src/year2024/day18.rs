use std::{
	collections::VecDeque,
	io::{BufRead, BufReader, Lines},
};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

fn bfs(map: &Matrix<bool>) -> Option<usize> {
	let start_position = Position { x: 0, y: 0 };
	let end_position = Position {
		x: WIDTH as isize - 1,
		y: HEIGHT as isize - 1,
	};

	let mut visited = Matrix::new(WIDTH as u8, HEIGHT as u8, vec![false; WIDTH * HEIGHT]);
	let mut costs = Matrix::new(WIDTH as u8, HEIGHT as u8, vec![0; WIDTH * HEIGHT]);
	let mut queue = VecDeque::with_capacity(WIDTH * HEIGHT);

	queue.push_front(start_position);

	while let Some(pos) = queue.pop_back() {
		visited.set(pos.x, pos.y, true);

		if pos == end_position {
			return costs.get(pos.x, pos.y);
		}

		for direction in Direction::all() {
			let next_pos = direction.update_position(pos);

			if !map.get(next_pos.x, next_pos.y).unwrap_or(true)
				&& !visited.get(next_pos.x, next_pos.y).unwrap_or(true)
			{
				visited.set(next_pos.x, next_pos.y, true);
				costs.set(next_pos.x, next_pos.y, costs.get(pos.x, pos.y).unwrap() + 1);
				queue.push_front(next_pos);
			}
		}
	}

	None
}

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	const CALCULATE_AT: usize = 1023;
	let mut map: Matrix<bool> = Matrix::new(WIDTH as u8, HEIGHT as u8, vec![false; WIDTH * HEIGHT]);

	let mut p1 = 0;

	for (i, line) in reader.lines().enumerate() {
		let line = line.unwrap();
		let mut parts = line.split(",");

		let x = parts.next().unwrap().parse().unwrap();
		let y = parts.next().unwrap().parse().unwrap();

		map.set(x, y, true);

		if i == CALCULATE_AT {
			p1 = bfs(&map).unwrap();
		} else if i > CALCULATE_AT {
			let result = bfs(&map);
			if result.is_none() {
				println!("Part 2: {},{}", x, y);
				println!("Index: {}", i);
				break;
			}
		}
	}

	(p1 as i64, -1)
}
