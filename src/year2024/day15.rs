use std::{collections::HashMap, io::BufRead};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum MapPart {
	Wall,
	Box,
	Empty,
	LeftBox,
	RightBox,
}

fn print_map(map: &Matrix<MapPart>, robot_pos: &Position) {
	for y in 0..map.height {
		for x in 0..map.width {
			if x as isize == robot_pos.x && y as isize == robot_pos.y {
				print!("@");
				continue;
			}
			print!(
				"{}",
				match map.get(x as isize, y as isize) {
					Some(MapPart::Wall) => '#',
					Some(MapPart::Box) => 'O',
					Some(MapPart::Empty) => '.',
					Some(MapPart::LeftBox) => '[',
					Some(MapPart::RightBox) => ']',
					None => ' ',
				}
			);
		}
		println!();
	}
}

fn part_1(
	map: &Matrix<MapPart>,
	mut robot_pos: Position,
	directions: &Vec<Direction>,
) -> Matrix<MapPart> {
	let mut map = map.clone();

	for dir in directions {
		let mut stacks = 0;
		let mut current_pos = robot_pos;
		loop {
			current_pos = dir.update_position(current_pos);
			match map.get(current_pos.x, current_pos.y) {
				Some(MapPart::Empty) => {
					break;
				}
				Some(MapPart::Box) => {
					stacks += 1;
				}
				Some(MapPart::Wall) | None => {
					stacks = -1;
					break;
				}
				_ => panic!(
					"Invalid map character: {:?}",
					map.get(current_pos.x, current_pos.y)
				),
			}
		}

		let next_pos = dir.update_position(robot_pos);

		if stacks >= 0 {
			robot_pos = next_pos;
			if stacks > 0 {
				map.set(next_pos.x, next_pos.y, MapPart::Empty);
				map.set(current_pos.x, current_pos.y, MapPart::Box);
			}
		}
	}

	map
}

fn part_2(
	map: &Matrix<MapPart>,
	robot_pos: Position,
	directions: &Vec<Direction>,
) -> Matrix<MapPart> {
	let mut data = Vec::with_capacity(map.data.len() * 2);

	for part in map.data.iter() {
		match part {
			MapPart::Wall => {
				data.push(MapPart::Wall);
				data.push(MapPart::Wall);
			}
			MapPart::Box => {
				data.push(MapPart::LeftBox);
				data.push(MapPart::RightBox);
			}
			MapPart::Empty => {
				data.push(MapPart::Empty);
				data.push(MapPart::Empty);
			}
			_ => panic!("Invalid map character: {:?}", part),
		}
	}

	let mut map = Matrix::new(map.width * 2, map.height, data);

	let mut robot_pos = Position {
		x: robot_pos.x * 2,
		y: robot_pos.y,
	};
	let mut queue = HashMap::new();

	for dir in directions {
		queue.clear();

		fn can_move(
			map: &Matrix<MapPart>,
			queue: &mut HashMap<Position, MapPart>,
			dir: Direction,
			current_pos: Position,
		) -> bool {
			let next_pos = dir.update_position(current_pos);
			let check_move = match (dir, map.get(next_pos.x, next_pos.y)) {
				(
					Direction::Left | Direction::Right,
					Some(MapPart::LeftBox) | Some(MapPart::RightBox),
				) => can_move(map, queue, dir, next_pos),
				(_, Some(MapPart::LeftBox)) => {
					can_move(map, queue, dir, next_pos)
						&& can_move(map, queue, dir, Direction::Right.update_position(next_pos))
				}
				(_, Some(MapPart::RightBox)) => {
					can_move(map, queue, dir, next_pos)
						&& can_move(map, queue, dir, Direction::Left.update_position(next_pos))
				}
				(_, Some(MapPart::Empty)) => true,
				_ => false,
			};

			if check_move {
				queue.insert(next_pos, map.get(current_pos.x, current_pos.y).unwrap());
				if !queue.contains_key(&current_pos) {
					queue.insert(current_pos, MapPart::Empty);
				}
			}

			check_move
		}

		if can_move(&map, &mut queue, *dir, robot_pos) {
			robot_pos = dir.update_position(robot_pos);
			for (pos, part) in queue.iter() {
				map.set(pos.x, pos.y, *part);
			}
		}
	}

	map
}

fn calculate_gps(map: &Matrix<MapPart>) -> i64 {
	let mut sum = 0;
	for y in 0..map.height as isize {
		for x in 0..map.width as isize {
			match map.get(x, y) {
				Some(MapPart::Box) | Some(MapPart::LeftBox) => sum += 100 * y + x,
				_ => {}
			}
		}
	}
	sum as i64
}

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	let mut data = Vec::new();
	let mut width = 0;
	let mut height = 0;

	let mut robot_pos: Option<Position> = None;
	let mut map: Option<Matrix<MapPart>> = None;
	let mut directions: Vec<Direction> = Vec::new();

	for line in reader.lines() {
		let line = line.unwrap();
		if map.is_some() {
			for char in line.chars() {
				let dir: Direction = match char {
					'^' => Direction::Up,
					'>' => Direction::Right,
					'v' => Direction::Down,
					'<' => Direction::Left,
					_ => continue,
				};

				directions.push(dir);
			}
		} else if line.is_empty() {
			if robot_pos.is_none() {
				panic!("No robot position found");
			}
			let _map = Matrix::new(width, height, data.clone());
			map = Some(_map);
		} else {
			height += 1;
			width = 0;
			for char in line.chars() {
				width += 1;
				match char {
					'#' => data.push(MapPart::Wall),
					'.' => data.push(MapPart::Empty),
					'O' => data.push(MapPart::Box),
					'@' => {
						robot_pos = Some(Position {
							x: width as isize - 1,
							y: height as isize - 1,
						});
						data.push(MapPart::Empty)
					}
					_ => panic!("Invalid map character: {}", char),
				}
			}
		}
	}

	if map.is_none() {
		panic!("Invalid input");
	}

	let map = map.unwrap();
	let robot_pos = robot_pos.unwrap();

	let map1 = part_1(&map, robot_pos, &directions);
	let map2 = part_2(&map, robot_pos, &directions);

	(calculate_gps(&map1), calculate_gps(&map2))
}
