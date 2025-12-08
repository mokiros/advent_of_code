use std::{collections::HashMap, io::BufRead, str::Chars};

use itertools::Itertools;

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	type Computer = usize;
	let tonum = |chars: &mut Chars| {
		let c1 = chars.next().unwrap() as usize - b'a' as usize;
		let c2 = chars.next().unwrap() as usize - b'a' as usize;
		c1 * 26 + c2
	};

	let mut connections: HashMap<Computer, Vec<Computer>> = HashMap::with_capacity(1000);
	let mut edges = vec![[false; 26 * 26]; 26 * 26];

	let lines = reader.lines();

	for line in lines {
		let line = line.unwrap();
		let mut chars = line.chars();
		let c1: Computer = tonum(&mut chars);
		chars.next();
		let c2: Computer = tonum(&mut chars);

		connections.entry(c1).or_default().push(c2);
		connections.entry(c2).or_default().push(c1);

		edges[c1][c2] = true;
		edges[c2][c1] = true;
	}

	// part 1
	let mut p1 = 0;
	let mut seen = vec![false; 26 * 26];
	const START_T: usize = 26 * (b't' - b'a') as usize;
	const END_T: usize = START_T + 26;

	for c1 in START_T..END_T {
		if let Some(cons) = connections.get(&c1) {
			seen[c1] = true;
			for combo in cons.iter().combinations(2) {
				let c2 = *combo[0];
				let c3 = *combo[1];
				if !seen[c2] && !seen[c3] && edges[c2][c3] {
					p1 += 1;
				}
			}
		}
	}

	// part 2
	let mut seen = [false; 676];
	let mut clique = Vec::new();
	let mut largest = Vec::new();

	for (c1, cons) in connections {
		if !seen[c1] {
			clique.clear();
			clique.push(c1);

			for c2 in cons {
				if clique.iter().all(|c| edges[c2][*c]) {
					seen[c2] = true;
					clique.push(c2);
				}
			}

			if clique.len() > largest.len() {
				largest.clone_from(&clique);
			}
		}
	}

	largest.sort_unstable();

	println!(
		"Part 2: {}",
		largest
			.iter()
			.map(|c| {
				format!(
					"{}{}",
					(b'a' + (c / 26) as u8) as char,
					(b'a' + (c % 26) as u8) as char
				)
			})
			.join(",")
	);

	(p1.to_string(), (-1).to_string())
}
