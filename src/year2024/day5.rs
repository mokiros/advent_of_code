use std::io::{BufRead, Seek};

fn part_1<R: BufRead>(reader: &mut R) -> usize {
	let mut is_order = true;
	let mut count = 0;
	let mut ordering = vec![vec![false; 100]; 100];

	'lines: for line in reader.lines() {
		let line = line.expect("Unable to read line");

		if line.is_empty() {
			is_order = false;
			continue;
		}

		if is_order {
			let mut parts = line.split("|");
			let a = parts
				.next()
				.expect("Unable to parse a")
				.parse::<usize>()
				.expect("Unable to parse a");
			let b = parts
				.next()
				.expect("Unable to parse b")
				.parse::<usize>()
				.expect("Unable to parse b");
			ordering[b][a] = true;
			continue;
		}

		let pages = line
			.split(",")
			.map(|x| x.parse::<usize>().expect("Unable to parse number"))
			.collect::<Vec<usize>>();

		for i in 0..pages.len() - 1 {
			let a = pages[i];
			let b = pages[i + 1];
			if ordering[a][b] {
				continue 'lines;
			}
		}

		count += pages[pages.len() / 2];
	}

	return count;
}

fn part_2<R: BufRead>(reader: &mut R) -> usize {
	let mut is_order = true;
	let mut ordering = vec![vec![false; 100]; 100];
	let mut count = 0;

	for line in reader.lines() {
		let line = line.expect("Unable to read line");

		if line.is_empty() {
			is_order = false;
			continue;
		}

		if is_order {
			let mut parts = line.split("|");
			let a = parts
				.next()
				.expect("Unable to parse a")
				.parse::<usize>()
				.expect("Unable to parse a");
			let b = parts
				.next()
				.expect("Unable to parse b")
				.parse::<usize>()
				.expect("Unable to parse b");
			ordering[b][a] = true;
			continue;
		}

		let pages = line
			.split(",")
			.map(|x| x.parse::<usize>().expect("Unable to parse number"))
			.collect::<Vec<usize>>();

		for i in 0..pages.len() - 1 {
			if ordering[pages[i]][pages[i + 1]] {
				for page in pages.iter() {
					let mut sum = 0;
					for n in pages.iter() {
						if *n != *page && ordering[*n][*page] {
							sum += 1;
						}
					}
					if sum == pages.len() / 2 {
						count += page;
					}
				}
				break;
			}
		}
	}

	return count;
}

pub fn solve<R: BufRead + Seek>(reader: &mut R) -> (i64, i64) {
	let p1 = part_1(reader);

	reader.rewind().expect("Unable to rewind");

	let p2 = part_2(reader);

	(p1 as i64, p2 as i64)
}
