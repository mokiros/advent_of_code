use std::{
	collections::HashMap,
	io::{BufRead, Seek},
};

pub fn solve<R: BufRead + Seek>(reader: R) -> (String, String) {
	let mut part1: u64 = 0;

	let mut beams: HashMap<u64, u64> = HashMap::new();

	let mut x = 0;
	for byte in reader.bytes() {
		let byte = byte.unwrap();

		match byte {
			b'S' => {
				beams.insert(x, 1);
			}
			b'^' => {
				if let Some(count) = beams.remove(&x) {
					beams
						.entry(x - 1)
						.and_modify(|a| *a += count)
						.or_insert(count);
					beams
						.entry(x + 1)
						.and_modify(|a| *a += count)
						.or_insert(count);
					part1 += 1;
				}
				x += 1;
			}
			b'\n' => {
				x = 0;
			}
			b'.' => {
				x += 1;
			}
			byte => panic!("Invalid character: {}", byte as char),
		}
	}

	let part2: u64 = beams.values().fold(0, |sum, count| sum + *count);

	(part1.to_string(), part2.to_string())
}
