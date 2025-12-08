use std::{
	collections::HashSet,
	io::{BufRead, Seek},
};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

#[derive(Debug, Clone, Copy)]
enum MapPart {
	Empty,
	Wall,
	Visited,
}

fn read_map<R: BufRead>(reader: &mut R) -> (Matrix<MapPart>, Position) {
	let mut width: u8 = 0;
	let mut height: u8 = 0;
	let mut data: Vec<MapPart> = Vec::new();
	let mut guard_position: Option<Position> = None;
	for (y, line) in reader.lines().enumerate() {
		let line = line.unwrap();
		width = u8::try_from(line.len()).unwrap();
		height += 1;

		for (x, char) in line.chars().enumerate() {
			let part = match char {
				'#' => MapPart::Wall,
				'.' => MapPart::Empty,
				'^' => {
					guard_position = Some(Position {
						x: x.cast_signed(),
						y: y.cast_signed(),
					});
					MapPart::Empty
				}
				_ => panic!("Invalid map character: {char}"),
			};
			data.push(part);
		}
	}

	(
		Matrix::new(width, height, data),
		guard_position.expect("Could not find guard position"),
	)
}

fn check_loop(map: &Matrix<MapPart>, mut pos: Position, mut dir: Direction) -> bool {
	let mut hits: HashSet<(Position, Direction)> = HashSet::new();

	for _ in 0..(map.width as usize * map.height as usize) {
		let next_pos = dir.update_position(&pos);

		match map.get(next_pos.x, next_pos.y) {
			Some(MapPart::Wall) => {
				if !hits.insert((pos, dir)) {
					return true;
				}
				dir = dir.next();
			}
			Some(_) => {
				pos = next_pos;
			}
			None => {
				return false;
			}
		}
	}

	panic!("loop check: Did not reach the edge of the map");
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let (mut map, mut guard_position) = read_map(&mut reader);

	let mut current_direction = Direction::Up;

	let mut count = 0;
	let mut loop_count = 0;

	for _ in 0..(map.width as usize * map.height as usize) {
		let next_pos = current_direction.update_position(&guard_position);

		match map.get(next_pos.x, next_pos.y) {
			Some(MapPart::Wall) => {
				current_direction = current_direction.next();
			}
			Some(MapPart::Empty) => {
				map.set(next_pos.x, next_pos.y, MapPart::Wall);
				let looped = check_loop(&map, guard_position, current_direction.next());
				if looped {
					loop_count += 1;
				}
				map.set(next_pos.x, next_pos.y, MapPart::Visited);
				guard_position = next_pos;
				count += 1;
			}
			Some(MapPart::Visited) => {
				guard_position = next_pos;
			}
			None => {
				return (count.to_string(), loop_count.to_string());
			}
		}
	}

	panic!("Did not reach the edge of the map");
}
