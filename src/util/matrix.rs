#[derive(Debug, Clone)]
pub struct Matrix<T> {
	pub width: u8,
	pub height: u8,
	pub data: Vec<T>,
}

impl<T: Copy> Matrix<T> {
	pub fn new(width: u8, height: u8, data: Vec<T>) -> Self {
		Self {
			data,
			width,
			height,
		}
	}

	fn get_idx(&self, x: isize, y: isize) -> Option<usize> {
		let w = self.width as isize;
		let h = self.height as isize;
		if x < w && y < h && x >= 0 && y >= 0 {
			Some((y * w + x) as usize)
		} else {
			None
		}
	}

	pub fn get(&self, x: isize, y: isize) -> Option<T> {
		self.get_idx(x, y).map(|idx| self.data[idx])
	}

	pub fn set(&mut self, x: isize, y: isize, value: T) {
		if let Some(idx) = self.get_idx(x, y) {
			self.data[idx] = value;
		} else {
			panic!("Matrix index out of bounds: {} {}", x, y)
		}
	}
}
