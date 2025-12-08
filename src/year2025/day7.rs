pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut part1: u64 = 0;

	let mut beams = [0_u64; 150];

	let mut x = 0;
	let mut i = 0;
	while i < buffer.len() {
		let byte = buffer[i];
		i += 1;
		x += 1;

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
				i += x; // skip every other line
				x = 0;
			}
			_ => {}
		}
	}

	let part2: u64 = beams.iter().sum();

	(part1.to_string(), part2.to_string())
}
