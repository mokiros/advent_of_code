use std::io::{BufRead, Read, Seek};

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let mut map: Vec<Option<usize>> = Vec::new();

	for (i, byte) in reader.bytes().enumerate() {
		let ch = byte.unwrap() as char;
		if ch == '\n' {
			break;
		}
		let length = ch.to_digit(10).unwrap();
		let element = match i % 2 {
			0 => Some(i / 2),
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
		let n = if let Some(n) = map[i] {
			n
		} else {
			while end > i && map[end].is_none() {
				end -= 1;
			}
			if end <= i {
				break;
			}
			let n = map[end].unwrap();
			end -= 1;
			n
		};
		p1 += n * i;
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
			if let Some(n) = map[end] {
				id = n;
				length = 1;
			} else {
				id = 0;
				length = 0;
			}
		}
	}

	let p2 = map
		.iter()
		.enumerate()
		.fold(0, |acc, (i, v)| v.as_ref().map_or(acc, |n| acc + n * i));

	(p1.to_string(), p2.to_string())
}
