use std::{
	cmp::Ordering,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Vector3 {
	x: u64,
	y: u64,
	z: u64,
}

impl Vector3 {
	#[inline]
	const fn distance_squared(self, other: Self) -> u64 {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		let dz = self.z - other.z;
		dx * dx + dy * dy + dz * dz
	}
}

impl From<(u64, u64, u64)> for Vector3 {
	fn from(value: (u64, u64, u64)) -> Self {
		Self {
			x: value.0,
			y: value.1,
			z: value.2,
		}
	}
}

struct Dsu {
	parent: Vec<usize>,
	size: Vec<usize>,
}

impl Dsu {
	fn new(n: usize) -> Self {
		Self {
			parent: (0..n).collect(),
			size: vec![1; n],
		}
	}

	fn find(&mut self, i: usize) -> usize {
		if self.parent[i] != i {
			self.parent[i] = self.find(self.parent[i]);
		}
		self.parent[i]
	}

	fn connect(&mut self, i: usize, j: usize) -> bool {
		let root_i = self.find(i);
		let root_j = self.find(j);

		if root_i == root_j {
			false
		} else {
			if self.size[root_i] < self.size[root_j] {
				self.parent[root_i] = root_j;
				self.size[root_j] += self.size[root_i];
			} else {
				self.parent[root_j] = root_i;
				self.size[root_i] += self.size[root_j];
			}
			true
		}
	}

	fn get_component_sizes(&self) -> Vec<usize> {
		self.parent
			.iter()
			.enumerate()
			.filter(|&(i, &p)| i == p)
			.map(|(i, _)| self.size[i])
			.collect()
	}
}

struct Pair {
	a: usize,
	b: usize,
	d: u64,
}

impl PartialEq for Pair {
	fn eq(&self, other: &Self) -> bool {
		(self.a.eq(&other.a) && self.b.eq(&other.b)) || (self.a.eq(&other.b) && self.b.eq(&other.a))
	}
}

impl PartialOrd for Pair {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.d.partial_cmp(&other.d)
	}
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut part1: u64 = 0;
	let mut part2: u64 = 0;

	let mut boxes: Vec<Vector3> = Vec::with_capacity(1000);

	let mut coords_buffer = [0_u64; 2];
	let mut val = 0;
	let mut idx = 0;

	for b in buffer {
		match b {
			b',' => {
				coords_buffer[idx] = val;
				idx += 1;
				val = 0;
			}
			b'\n' => {
				if idx != 0 {
					boxes.push(Vector3 {
						x: coords_buffer[0],
						y: coords_buffer[1],
						z: val,
					});
					idx = 0;
				}
				val = 0;
			}
			_ => {
				val = val * 10 + u64::from(b - b'0');
			}
		}
	}

	let n = boxes.len();

	let mut pairs: Vec<Pair> = Vec::with_capacity(n * (n - 1) / 2);
	for a in 0..n {
		for b in (a + 1)..n {
			let d = boxes[a].distance_squared(boxes[b]);
			pairs.push(Pair { a, b, d });
		}
	}
	pairs.sort_unstable_by_key(|e| e.d);

	let part1_target_size = if n == 10 { 10 } else { 1000 };

	let mut dsu = Dsu::new(n);
	let mut num_circuits = n;
	for (i, pair) in pairs.iter().enumerate() {
		if i == part1_target_size {
			// part 1
			let mut sizes = dsu.get_component_sizes();
			sizes.sort_unstable_by(|a, b| b.cmp(a));
			part1 = sizes.iter().take(3).fold(1, |a, b| a * (*b) as u64);
		}
		if dsu.connect(pair.a, pair.b) {
			num_circuits -= 1;

			// for part 2, keep connecting until the last circuit
			if num_circuits == 1 {
				part2 = boxes[pair.a].x * boxes[pair.b].x;
				break;
			}
		}
	}

	(part1.to_string(), part2.to_string())
}
