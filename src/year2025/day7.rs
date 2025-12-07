use std::io::{BufRead, Seek};

pub fn solve<R: BufRead + Seek>(reader: R) -> (String, String) {
	let mut part1: u64 = 0;

	let mut beams = [0_u64; 150];

	let mut x = 1;
	let mut bytes = reader.bytes();
	while let Some(Ok(byte)) = bytes.next() {
		match byte {
			b'S' => {
				beams[x] = 1;
			}
			b'^' => {
				let count = beams[x];
				if count != 0 {
					beams[x] = 0;
					beams[x - 1] += count;
					beams[x + 1] += count;
					part1 += 1;
				}
			}
			b'\n' => {
				bytes.nth(x - 1); // skip every other line
				x = 1;
				continue;
			}
			b'.' => {}
			byte => panic!("Invalid character: {}", byte as char),
		}

		x += 1;
	}

	let part2: u64 = beams.iter().sum();

	(part1.to_string(), part2.to_string())
}
