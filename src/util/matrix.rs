use std::io::BufRead;

use super::position::Position;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
	pub width: u8,
	pub height: u8,
	pub data: Vec<T>,
}

#[allow(unused)]
impl<T: Copy> Matrix<T> {
	pub const fn new(width: u8, height: u8, data: Vec<T>) -> Self {
		Self {
			width,
			height,
			data,
		}
	}

	pub fn read_map_with_start_and_end<R: BufRead>(
		reader: R,
		char_fn: fn(char: char) -> T,
	) -> (Self, Position, Position) {
		let mut start_position: Option<Position> = Some(Position { x: 0, y: 0 });
		let mut end_position: Option<Position> = Some(Position { x: 0, y: 0 });

		let mut data: Vec<T> = Vec::new();
		let mut width = 0;
		let mut height = 0;

		for line in reader.lines() {
			let line = line.unwrap();
			width = 0;
			height += 1;

			for char in line.chars() {
				width += 1;
				data.push(char_fn(char));
				match char {
					'S' => {
						start_position = Some(Position {
							x: width - 1,
							y: height - 1,
						});
					}
					'E' => {
						end_position = Some(Position {
							x: width - 1,
							y: height - 1,
						});
					}
					_ => (),
				}
			}
		}

		(
			Self::new(width.try_into().unwrap(), height.try_into().unwrap(), data),
			start_position.unwrap(),
			end_position.unwrap(),
		)
	}

	const fn get_idx(&self, x: isize, y: isize) -> Option<usize> {
		let w = self.width as isize;
		let h = self.height as isize;
		if x < w && y < h && x >= 0 && y >= 0 {
			Some((y * w + x).cast_unsigned())
		} else {
			None
		}
	}

	pub fn get(&self, x: isize, y: isize) -> Option<T> {
		self.get_idx(x, y).map(|idx| self.data[idx])
	}

	pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
		self.get_idx(x, y).map(|idx| &mut self.data[idx])
	}

	pub fn set(&mut self, x: isize, y: isize, value: T) {
		if let Some(idx) = self.get_idx(x, y) {
			self.data[idx] = value;
		} else {
			panic!("Matrix index out of bounds: {x} {y}")
		}
	}

	pub fn to_string(&self, char_fn: fn(element: &T) -> char) -> String {
		let mut s = String::with_capacity(self.width as usize * self.height as usize);
		for y in 0..self.height {
			for x in 0..self.width {
				s.push(char_fn(&self.get(x as isize, y as isize).unwrap()));
			}
			s.push('\n');
		}
		s
	}
}
