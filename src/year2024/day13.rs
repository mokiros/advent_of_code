use std::io::{BufRead, Seek};

const fn calc(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> Option<i64> {
	let n1 = x1 * y3 - y1 * x3;
	let n2 = y2 * x3 - x2 * y3;
	let div = x1 * y2 - y1 * x2;

	if div == 0 || n1 % div != 0 || n2 % div != 0 {
		return None;
	}

	let a = n1 / div;
	let b = n2 / div;

	if a > 0 && b > 0 {
		Some(a + 3 * b)
	} else {
		None
	}
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let re = regex::Regex::new(r"(\d+)").unwrap();
	let mut numbers = Vec::with_capacity(6);

	let mut p1 = 0;
	let mut p2 = 0;

	for line in reader.lines() {
		for cap in re.captures_iter(&line.unwrap()) {
			let n: i64 = cap.get(1).unwrap().as_str().parse().unwrap();
			numbers.push(n);
			if numbers.len() == 6 {
				let x1 = numbers[0];
				let y1 = numbers[1];
				let x2 = numbers[2];
				let y2 = numbers[3];
				let x3 = numbers[4];
				let y3 = numbers[5];

				numbers.clear();

				if let Some(n) = calc(x1, y1, x2, y2, x3, y3) {
					p1 += n;
				}
				if let Some(n) = calc(
					x1,
					y1,
					x2,
					y2,
					x3 + 10_000_000_000_000,
					y3 + 10_000_000_000_000,
				) {
					p2 += n;
				}
			}
		}
	}

	(p1.to_string(), p2.to_string())
}
