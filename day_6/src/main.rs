use std::{
	collections::{HashMap, HashSet},
	fs::File,
	io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone, Copy)]
enum MapPart {
	Empty,
	Wall,
	Visited,
}

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
	Top,
	Right,
	Bottom,
	Left,
}

impl Direction {
	fn next(&self) -> Self {
		match self {
			Self::Top => Self::Right,
			Self::Right => Self::Bottom,
			Self::Bottom => Self::Left,
			Self::Left => Self::Top,
		}
	}

	fn update_position(&self, pos: Position) -> Position {
		match self {
			Self::Top => Position {
				x: pos.x,
				y: pos.y - 1,
			},
			Self::Right => Position {
				x: pos.x + 1,
				y: pos.y,
			},
			Self::Bottom => Position {
				x: pos.x,
				y: pos.y + 1,
			},
			Self::Left => Position {
				x: pos.x - 1,
				y: pos.y,
			},
		}
	}
}

fn read_map<R: Read>(reader: &mut BufReader<R>) -> (Matrix<MapPart>, Position) {
	let mut width: u8 = 0;
	let mut height: u8 = 0;
	let mut data: Vec<MapPart> = Vec::new();
	let mut guard_position: Option<Position> = None;
	for (y, line) in reader.lines().enumerate() {
		let line = line.unwrap();
		width = line.len() as u8;
		height += 1;

		for (x, char) in line.chars().enumerate() {
			let part = match char {
				'#' => MapPart::Wall,
				'.' => MapPart::Empty,
				'^' => {
					guard_position = Some(Position {
						x: x as u8 + 1,
						y: y as u8 + 1,
					});
					MapPart::Empty
				}
				_ => panic!("Invalid map character: {}", char),
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
		let next_pos = dir.update_position(pos);

		match map.get(next_pos) {
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

fn solve(reader: &mut BufReader<File>) {
	let (mut map, mut guard_position) = read_map(reader);

	let mut current_direction = Direction::Top;

	let mut count = 0;
	let mut loop_count = 0;

	for _ in 0..(map.width as usize * map.height as usize) {
		let current_tile = map.get(guard_position).expect(
			format!(
				"Guard is at the edge {} {}",
				guard_position.x, guard_position.y
			)
			.as_str(),
		);

		let next_pos = current_direction.update_position(guard_position);

		match map.get(next_pos) {
			Some(MapPart::Wall) => {
				current_direction = current_direction.next();
			}
			Some(MapPart::Empty) => {
				map.set(next_pos, MapPart::Wall);
				let looped = check_loop(&map, guard_position, current_direction.next());
				if looped {
					loop_count += 1;
				}
				map.set(next_pos, MapPart::Visited);
				guard_position = next_pos;
				count += 1;
			}
			Some(MapPart::Visited) => {
				guard_position = next_pos;
			}
			None => {
				println!("Part 1: {}\nPart 2: {}", count, loop_count);
				return;
			}
		}
	}

	panic!("Did not reach the edge of the map");
}

fn main() {
	let file = File::open("input.txt").expect("Unable to open file");
	let mut reader = BufReader::new(file);

	solve(&mut reader);
}
