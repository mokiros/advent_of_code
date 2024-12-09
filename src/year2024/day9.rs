use std::io::{BufRead, Seek};

pub fn solve<R: BufRead + Seek>(reader: R) -> (i64, i64) {
	let mut map: Vec<Option<usize>> = Vec::new();

	for (i, byte) in reader.bytes().enumerate() {
		let ch = byte.unwrap() as char;
		if ch == '\n' {
			break;
		}
		let length = ch.to_digit(10).unwrap();
		let element = match i % 2 {
			0 => Some(i / 2),
			1 => None,
			_ => None,
		};

		for _ in 0..length {
			map.push(element);
		}
	}

	// part 1
	let mut end = map.len() - 1;
	let mut i = 0;
	let mut p1 = 0;
	while i <= end {
		let n = match map[i] {
			Some(n) => n,
			None => {
				while end > i && map[end] == None {
					end -= 1;
				}
				if end <= i {
					break;
				}
				let n = map[end].unwrap();
				end -= 1;
				n
			}
		};
		p1 += (n as usize) * i;
		i += 1;
	}

	// part 2
	let mut end = map.len();
	let mut id = 0;
	let mut length = 0;
	while end > 0 {
		end -= 1;
		if map[end] == Some(id) {
			length += 1;
		} else {
			if length > 0 {
				let mut empty_length = 0;
				for i in 0..=end {
					if map[i].is_none() {
						empty_length += 1;
						if empty_length == length {
							for j in 0..length {
								map[end + j + 1] = None;
								map[i - j] = Some(id);
							}
							break;
						}
					} else {
						empty_length = 0;
					}
				}
			}
			match map[end] {
				Some(n) => {
					id = n;
					length = 1;
				}
				None => {
					id = 0;
					length = 0;
				}
			}
		}
	}

	let p2 = map.iter().enumerate().fold(0, |acc, (i, v)| match v {
		Some(n) => acc + n * i,
		None => acc,
	});

	(p1 as i64, p2 as i64)
}
