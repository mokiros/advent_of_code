use std::{
	collections::{HashMap, HashSet},
	io::{BufRead, Seek},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
	x: i8,
	y: i8,
}

pub fn solve<R: BufRead + Seek>(reader: R) -> (i64, i64) {
	let mut width: i8 = 0;
	let mut height: i8 = 0;

	let mut node_positions: HashMap<char, Vec<Position>> = HashMap::new();

	for line in reader.lines() {
		height += 1;
		let line = line.unwrap();

		width = 0;
		for char in line.chars() {
			width += 1;
			if char == '.' {
				continue;
			}

			let position = Position {
				x: width,
				y: height,
			};
			node_positions.entry(char).or_default().push(position);
		}
	}

	let mut antinodes1: HashSet<Position> = HashSet::new();
	let mut antinodes2: HashSet<Position> = HashSet::new();

	for (_, positions) in node_positions.iter() {
		for p1 in positions.iter() {
			for p2 in positions.iter() {
				if p1 == p2 {
					continue;
				}

				let dx = p1.x - p2.x;
				let dy = p1.y - p2.y;
				let mut p3 = Position {
					x: p1.x,
					y: p1.y,
				};

				let mut i = 0;
				while !(p3.x < 1 || p3.x > width || p3.y < 1 || p3.y > height) {
					antinodes2.insert(p3.clone());
					p3.x += dx;
					p3.y += dy;
					if i == 0 {
						antinodes1.insert(p3.clone());
						i = 1;
					}
				}
			}
		}
	}

	(antinodes1.len() as i64, antinodes2.len() as i64)
}
