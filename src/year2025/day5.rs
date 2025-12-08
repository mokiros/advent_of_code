use std::io::{BufRead, Seek};

fn parse_num(vec: &[u8]) -> u64 {
	vec.iter().fold(0, |acc, &b| acc * 10 + u64::from(b - b'0'))
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let mut part1: u64 = 0;
	let mut part2: u64 = 0;

	let mut ranges: Vec<(u64, u64)> = Vec::with_capacity(200);
	let mut buf = Vec::with_capacity(16);

	let mut parsing_ranges = true;

	loop {
		buf.clear();

		if reader.read_until(b'\n', &mut buf).unwrap() == 0 {
			break;
		}

		let mut line = buf.as_slice();
		while let Some(&last) = line.last() {
			if last == b'\n' {
				line = &line[..line.len() - 1];
			} else {
				break;
			}
		}

		if line.is_empty() {
			parsing_ranges = false;
			continue;
		}

		if parsing_ranges {
			let dash_pos = line.iter().position(|&b| b == b'-').unwrap();

			let start = parse_num(&line[..dash_pos]);
			let end = parse_num(&line[dash_pos + 1..]);

			ranges.push((start, end));
			continue;
		}

		// part 1
		let num = parse_num(line);
		for (start, end) in &ranges {
			if num >= *start && num <= *end {
				part1 += 1;
				break;
			}
		}
	}

	// part 2
	ranges.sort_by(|a, b| a.0.cmp(&b.0));

	let len = ranges.len();

	let mut i = 1;
	while i < len {
		let first = &ranges[i - 1];
		let last = &ranges[i];

		if first.1 < last.0 {
			part2 += first.1 - first.0 + 1;
		} else if first.1 <= last.1 {
			part2 += last.0 - first.0;
		} else {
			part2 += last.0 - first.0;

			let first_end = first.1;
			let last = &mut ranges[i];
			last.1 = first_end;
		}

		i += 1;
	}

	let last_range = ranges[len - 1];
	part2 += last_range.1 - last_range.0 + 1;

	(part1.to_string(), part2.to_string())
}
