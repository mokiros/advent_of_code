use std::io::BufRead;

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	let mut lines = reader.lines();

	let mut p1 = 0;
	let mut p2 = 0;
	let first_line = lines.next().unwrap().unwrap();
	let towels: Vec<&str> = first_line.split(", ").collect();

	lines.next();

	for line in lines {
		let line = line.unwrap();

		let n = line.len();
		let mut counts: Vec<usize> = vec![0; n + 1];
		counts[0] = 1;

		for i in 1..=n {
			for towel in towels.iter() {
				let len = towel.len();
				if i >= len && towel == &&line[(i - len)..i] {
					counts[i] += counts[i - len];
				}
			}
		}

		let count = counts[n];
		p1 += (count != 0) as usize;
		p2 += count;
	}

	(p1 as i64, p2 as i64)
}
