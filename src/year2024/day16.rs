use std::{collections::VecDeque, io::BufRead};

use crate::util::{direction::Direction, matrix::Matrix, position::Position};

fn read_map<R: BufRead>(reader: R) -> (Matrix<bool>, Position, Position) {
	let mut start_position: Option<Position> = Some(Position { x: 0, y: 0 });
	let mut end_position: Option<Position> = Some(Position { x: 0, y: 0 });

	let mut data: Vec<bool> = Vec::new();
	let mut width = 0;
	let mut height = 0;

	for line in reader.lines() {
		let line = line.unwrap();
		width = 0;
		height += 1;

		for char in line.chars() {
			width += 1;
			data.push(char == '#');
			match char {
				'S' => {
					start_position = Some(Position {
						x: width - 1,
						y: height - 1,
					})
				}
				'E' => {
					end_position = Some(Position {
						x: width - 1,
						y: height - 1,
					})
				}
				_ => (),
			}
		}
	}

	(
		Matrix::new(width as u8, height as u8, data),
		start_position.unwrap(),
		end_position.unwrap(),
	)
}

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	let (map, start_pos, end_pos) = read_map(reader);

	let mut buckets = vec![Vec::new(); 1001];
	let mut seen = Matrix::new(
		map.width,
		map.height,
		vec![[i32::MAX; 4]; map.width as usize * map.height as usize],
	);
	let mut cost = 0;
	let mut lowest = i32::MAX;

	buckets[0].push((start_pos, Direction::Right));
	seen.get_mut(start_pos.x, start_pos.y).unwrap()[Direction::Right as usize] = 0;

	while lowest == i32::MAX {
		let index = (cost % 1001) as usize;

		while let Some((position, direction)) = buckets[index].pop() {
			if position == end_pos {
				lowest = cost;
				break;
			}

			let left = direction.previous();
			let right = direction.next();
			let next = [
				(direction.update_position(position), direction, cost + 1),
				(position, left, cost + 1000),
				(position, right, cost + 1000),
			];

			for (next_position, next_direction, next_cost) in next {
				if map.get(next_position.x, next_position.y).unwrap_or(true) != true
					&& next_cost
						< seen.get(next_position.x, next_position.y).unwrap()
							[next_direction as usize]
				{
					let index = (next_cost % 1001) as usize;
					buckets[index].push((next_position, next_direction));
					seen.get_mut(next_position.x, next_position.y).unwrap()
						[next_direction as usize] = next_cost;
				}
			}
		}

		cost += 1;
	}

	let mut todo = VecDeque::new();
	let mut path = Matrix::new(
		map.width,
		map.height,
		vec![false; map.width as usize * map.height as usize],
	);

	for direction in Direction::all() {
		if seen.get(end_pos.x, end_pos.y).unwrap()[direction as usize] == lowest {
			todo.push_back((end_pos, direction, lowest));
		}
	}

	while let Some((position, direction, cost)) = todo.pop_front() {
		path.set(position.x, position.y, true);

		if position == start_pos {
			continue;
		}

		let left = direction.previous();
		let right = direction.next();
		let next = [
			(
				direction.opposite().update_position(position),
				direction,
				cost - 1,
			),
			(position, left, cost - 1000),
			(position, right, cost - 1000),
		];

		for (next_position, next_direction, next_cost) in next {
			if next_cost
				== seen.get(next_position.x, next_position.y).unwrap()[next_direction as usize]
			{
				todo.push_back((next_position, next_direction, next_cost));
				seen.get_mut(next_position.x, next_position.y).unwrap()[next_direction as usize] =
					i32::MAX;
			}
		}
	}

	(
		lowest as i64,
		path.data.iter().filter(|&&b| b).count() as i64,
	)
}
