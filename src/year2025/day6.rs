use std::io::{BufRead, Read, Seek};

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let mut part1: u64 = 0;
	let mut part2: u64 = 0;

	let mut buf = Vec::new();
	reader.read_to_end(&mut buf).unwrap();

	let width = buf.iter().position(|b| *b == b'\n').unwrap() + 1;
	let rows = buf.len() / width - 1;

	let last_row = &buf[width * rows..(buf.len() - 1)];

	let mut nums1: Vec<u64> = vec![0; rows];
	let mut nums2: Vec<u64> = Vec::with_capacity(4);

	let mut iter = last_row.iter().enumerate().rev();
	while let Some((i, byte)) = iter.next() {
		let mut n2 = 0;

		for row in 0..rows {
			let byte = buf[width * row + i];
			if byte == b' ' {
				continue;
			}
			let digit = byte - b'0';
			let n1 = nums1[row];
			nums1[row] = if n1 == 0 {
				u64::from(digit)
			} else {
				u64::from(digit) * (10_u64.pow(n1.ilog10() + 1)) + n1
			};
			n2 = n2 * 10 + u64::from(digit);
		}

		if n2 != 0 {
			nums2.push(n2);
		}

		match *byte {
			b'*' => {
				part1 += nums1.iter().fold(1, |a, b| a * *b);
				part2 += nums2.iter().fold(1, |a, b| a * *b);
			}
			b'+' => {
				part1 += nums1.iter().sum::<u64>();
				part2 += nums2.iter().sum::<u64>();
			}
			b' ' => continue,
			byte => panic!("Unknown char: {}", byte as char),
		}

		nums1.fill(0);
		nums2.clear();

		iter.next();
	}

	(part1.to_string(), part2.to_string())
}
