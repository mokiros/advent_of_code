use std::io::BufRead;

pub fn solve<R: BufRead>(reader: R) -> (String, String) {
	let mut lines = reader.lines();

	let mut keys = Vec::new();
	let mut locks = Vec::new();

	while let Some(Ok(line)) = lines.next() {
		let is_key = line.starts_with('.');
		let mut data: [u8; 5] = [0; 5];
		for _ in 0..5 {
			let line = lines.next().unwrap().unwrap();
			for (i, char) in line.chars().enumerate() {
				if char == '#' {
					data[i] += 1;
				}
			}
		}
		lines.next();
		lines.next();

		if is_key {
			keys.push(data);
		} else {
			locks.push(data);
		}
	}

	let mut p1 = 0;

	for i in 0..keys.len() {
		let key: [u8; 5] = keys[i];
		'lock_loop: for j in 0..locks.len() {
			let lock = locks[j];
			for k in 0..5 {
				if key[k] + lock[k] > 5 {
					continue 'lock_loop;
				}
			}
			p1 += 1;
		}
	}

	(p1.to_string(), 0.to_string())
}
