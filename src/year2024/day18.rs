use std::{collections::VecDeque, io::BufRead};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

// Input does not contain map size, so we explicitly state it
const WIDTH: usize = 71;
const HEIGHT: usize = 71;

// Simple breadth first search
// Map contains values, indicating when the byte would fall
// so if we need to check at 1024th byte, we consider everything below 1024 as a wall
fn bfs(map: &Matrix<u16>, iteration: u16) -> Option<usize> {
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
			let next_pos = direction.update_position(&pos);

			let is_wall = map.get(next_pos.x, next_pos.y).unwrap_or(0) < iteration;
			let is_visited = visited.get(next_pos.x, next_pos.y).unwrap_or(true);

			if !is_wall && !is_visited {
				visited.set(next_pos.x, next_pos.y, true);
				costs.set(next_pos.x, next_pos.y, costs.get(pos.x, pos.y).unwrap() + 1);
				queue.push_front(next_pos);
			}
		}
	}

	None
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let mut map: Matrix<u16> =
		Matrix::new(WIDTH as u8, HEIGHT as u8, vec![u16::MAX; WIDTH * HEIGHT]);
	let mut bytes = Vec::new();

	let mut max: u16 = 0;

	// Read all positions at once
	for (i, line) in reader.lines().enumerate() {
		let line = line.unwrap();
		let mut parts = line.split(',');

		let x = parts.next().unwrap().parse().unwrap();
		let y = parts.next().unwrap().parse().unwrap();

		bytes.push(Position { x, y });
		map.set(x, y, i as u16);
		max = i as u16;
	}

	let p1 = bfs(&map, 1024).unwrap();
	let mut min: u16 = 1024;

	// binary search
	while min < max {
		let mid = u16::midpoint(min, max);
		if bfs(&map, mid).is_none() {
			max = mid;
		} else {
			min = mid + 1;
		}
	}

	let byte_idx = min as usize - 1;
	let pos = bytes[byte_idx];

	println!("Part 2: {},{}; Byte number: {}", pos.x, pos.y, byte_idx);

	(p1.to_string(), (-1).to_string())
}
