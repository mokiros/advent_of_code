use std::{collections::HashMap, io::BufRead};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	let (map, start_pos, end_pos) = Matrix::read_map_with_start_and_end(reader, |c| c == '#');

	let mut costs: HashMap<Position, isize> = HashMap::new();
	costs.insert(start_pos, 0);

	let mut current_position = start_pos;

	let mut path_length = 0;
	// Going over the path and recording the distance from the start
	loop {
		path_length += 1;
		if current_position == end_pos {
			break;
		}
		for dir in Direction::all() {
			let next_pos = dir.update_position(&current_position);
			if costs.contains_key(&next_pos) || map.get(next_pos.x, next_pos.y).unwrap_or(true) {
				continue;
			}
			costs.insert(next_pos, path_length);
			current_position = next_pos;
			break;
		}
	}

	// 40x40 lookup with a max distance of 20
	let mut lookup: Vec<Position> = Vec::new();
	const MAX_DISTANCE: isize = 20;
	for x in -MAX_DISTANCE..=MAX_DISTANCE {
		for y in -MAX_DISTANCE..=MAX_DISTANCE {
			let pos = Position { x, y };
			let total_distance = pos.manhattan_length();
			if total_distance >= 2 && total_distance <= MAX_DISTANCE {
				lookup.push(pos);
			}
		}
	}

	let mut p1 = 0;
	let mut p2 = 0;

	const MIN_SAVE: isize = 100;

	// applying the lookup above to every position on the path
	for (pos, cost) in costs.iter() {
		// No point in checking last 100 tiles of the path
		if path_length - cost < MIN_SAVE {
			continue;
		}
		for vec in lookup.iter() {
			let next_pos = pos + vec;
			let vec_distance = vec.manhattan_length();
			if let Some(new_cost) = costs.get(&next_pos) {
				let diff = new_cost - cost - vec_distance;
				if diff >= MIN_SAVE {
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
