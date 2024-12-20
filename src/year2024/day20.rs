use std::{collections::HashMap, io::BufRead};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	let (map, start_pos, end_pos) = Matrix::read_map_with_start_and_end(reader, |c| c == '#');

	let mut p1 = 0;
	let mut p2 = 0;

	let mut costs: HashMap<Position, i32> = HashMap::new();
	costs.insert(start_pos, 0);
	let mut current_position = start_pos;
	let mut step = 0;
	loop {
		step += 1;
		if current_position == end_pos {
			break;
		}
		for dir in Direction::all() {
			let next_pos = dir.update_position(&current_position);
			if costs.contains_key(&next_pos) || map.get(next_pos.x, next_pos.y).unwrap_or(true) {
				continue;
			}
			costs.insert(next_pos, step);
			current_position = next_pos;
			break;
		}
	}

	let mut lookup: Vec<Position> = Vec::new();
	let max_distance = 20;
	for x in -max_distance..=max_distance {
		for y in -max_distance..=max_distance {
			let total_distance = (x as i32).abs() + (y as i32).abs();
			if total_distance >= 2 && total_distance <= max_distance {
				lookup.push(Position {
					x: x as isize,
					y: y as isize,
				});
			}
		}
	}

	for (pos, cost) in costs.iter() {
		for vec in lookup.iter() {
			let next_pos = Position {
				x: pos.x + vec.x,
				y: pos.y + vec.y,
			};
			let vec_distance = (vec.x as i32).abs() + (vec.y as i32).abs();
			if let Some(new_cost) = costs.get(&next_pos) {
				let diff = new_cost - cost - vec_distance;
				if diff >= 100 {
					if vec_distance <= 2 {
						p1 += 1;
					}
					p2 += 1;
				}
			}
		}
	}

	(p1, p2)
}
