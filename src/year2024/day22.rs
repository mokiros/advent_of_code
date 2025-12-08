use std::{
	collections::{HashMap, HashSet},
	io::BufRead,
};

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let mut p1 = 0;

	let mut encountered_changes: HashSet<[i8; 4]> = HashSet::new();
	let mut changes: HashMap<[i8; 4], u16> = HashMap::new();

	for line in reader.lines() {
		let mut n1: i64 = line.unwrap().parse().unwrap();

		let mut diffs: [i8; 4] = [0; 4];

		let mut prev_digit = 0;

		for i in 0..2000 {
			const BITS_24: i64 = 0xFFFFFF;
			n1 = ((n1 << 6) ^ n1) & BITS_24;
			n1 = ((n1 >> 5) ^ n1) & BITS_24;
			n1 = ((n1 << 11) ^ n1) & BITS_24;

			let digit = (n1 % 10) as i8;
			let diff = digit - prev_digit;
			prev_digit = digit;

			if i < 3 {
				diffs[i + 1] = diff;
				continue;
			}

			diffs[0] = diffs[1];
			diffs[1] = diffs[2];
			diffs[2] = diffs[3];
			diffs[3] = diff;

			if encountered_changes.insert(diffs) {
				let sum = changes.entry(diffs).or_insert(0);
				*sum += digit as u16;
			}
		}

		encountered_changes.clear();

		p1 += n1;
	}

	let p2 = i64::from(*changes.iter().map(|v| v.1).max().unwrap());

	(p1.to_string(), p2.to_string())
}
