use std::io::{BufRead, Seek};

struct Matrix {
	data: Vec<u8>,
	width: u16,
	height: u16,
}

impl Matrix {
	fn new(width: u16, height: u16, data: Vec<u8>) -> Self {
		Self {
			data,
			width,
			height,
		}
	}

	fn get(&self, x: isize, y: isize) -> Option<u8> {
		if x < self.width as isize && y < self.height as isize && x >= 0 && y >= 0 {
			Some(self.data[((y as u16) * self.width + (x as u16)) as usize])
		} else {
			None
		}
	}
}

fn part_1<R: BufRead>(reader: &mut R) -> i32 {
	let lines_iter = reader.lines();
	let mut data: Vec<u8> = Vec::with_capacity(140 * 140);

	for line in lines_iter {
		let line = line.expect("Unable to read line");
		for c in line.chars() {
			data.push(match c {
				'X' => 0,
				'M' => 1,
				'A' => 2,
				'S' => 3,
				_ => panic!("Invalid character {}", c),
			});
		}
	}

	let matrix = Matrix::new(140, 140, data);

	let mut count = 0;
	// haha nesting goes brrr
	for y in 0..140 {
		for x in 0..140 {
			if matrix.get(x, y) == Some(0) {
				for dx in -1..=1 {
					'dyloop: for dy in -1..=1 {
						if dx == 0 && dy == 0 {
							continue;
						}
						for i in 0..=3 {
							if matrix.get(x + dx * i, y + dy * i) != Some(i as u8) {
								continue 'dyloop;
							}
						}
						count += 1;
					}
				}
			}
		}
	}

	return count;
}

fn part_2<R: BufRead>(reader: &mut R) -> i32 {
	let lines_iter = reader.lines();
	let mut data: Vec<u8> = Vec::with_capacity(140 * 140);

	for line in lines_iter {
		let line = line.expect("Unable to read line");
		for c in line.chars() {
			data.push(match c {
				'X' => 0,
				'M' => 1,
				'A' => 2,
				'S' => 3,
				_ => panic!("Invalid character {}", c),
			});
		}
	}

	let matrix = Matrix::new(140, 140, data);

	let mut count = 0;
	for y in 0..140 {
		for x in 0..140 {
			if matrix.get(x, y) == Some(2) {
				let a = matrix.get(x - 1, y - 1).unwrap_or(0);
				let b = matrix.get(x + 1, y + 1).unwrap_or(0);
				let c = matrix.get(x - 1, y + 1).unwrap_or(0);
				let d = matrix.get(x + 1, y - 1).unwrap_or(0);

				let n1 = a * 10 + b;
				let n2 = c * 10 + d;

				if (n1 == 13 || n1 == 31) && (n2 == 13 || n2 == 31) {
					count += 1;
				}
			}
		}
	}

	return count;
}

pub fn solve<R: BufRead + Seek>(mut reader: R) -> (i64, i64) {
	let p1 = part_1(&mut reader);

	reader.rewind().expect("Unable to rewind");

	let p2 = part_2(&mut reader);

	(p1 as i64, p2 as i64)
}
