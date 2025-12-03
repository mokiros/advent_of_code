use std::io::{BufRead, Seek};

fn find_highest<const N: usize>(line: &[u8]) -> u64 {
	(0..N)
		.rev()
		.fold((0_usize, 0_u64), |(leftmost_index, sum), i| {
			let (idx, byte) = line
				.iter()
				.enumerate()
				.skip(leftmost_index)
				.rev()
				.skip(i)
				.max_by(|a, b| a.1.cmp(b.1))
				.unwrap();

			(
				idx + 1,
				sum + u64::from(*byte - b'0') * (10_u64.pow(i.try_into().unwrap())),
			)
		})
		.1
}

pub fn solve<R: BufRead + Seek>(reader: R) -> (String, String) {
	let mut part1: u64 = 0;
	let mut part2: u64 = 0;

	let mut bytes = reader.bytes();

	let mut line: Vec<u8> = Vec::with_capacity(100);

	while let Some(Ok(byte)) = bytes.next() {
		if byte == b'\n' {
			part1 += find_highest::<2>(&line);
			part2 += find_highest::<12>(&line);
			line.clear();
			continue;
		}
		line.push(byte);
	}

	(part1.to_string(), part2.to_string())
}
