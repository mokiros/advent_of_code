use std::io::{BufRead, Seek};

fn part_1(file: &str) -> i32 {
	let re = regex::Regex::new(r"mul\((\d+),(\d+)\)");

	let mut count = 0;

	// for each match in regex
	for cap in re.unwrap().captures_iter(file) {
		let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
		let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
		count += x * y;
	}

	count
}

fn part_2(file: &str) -> i32 {
	let re = regex::Regex::new(r"(mul|do|don't)\(((\d+),(\d+))?\)");

	let mut count = 0;

	let mut enabled = true;

	// for each match in regex
	for cap in re.unwrap().captures_iter(file) {
		let cmd = cap.get(1).unwrap().as_str();
		match cmd {
			"do" => enabled = true,
			"don't" => enabled = false,
			"mul" => {
				if enabled {
					let x = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
					let y = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
					count += x * y;
				}
			}
			_ => {}
		}
	}

	count
}

pub fn solve<R: BufRead + Seek>(mut reader: R) -> (String, String) {
	let mut file = String::new();
	reader.read_to_string(&mut file).unwrap();

	let p1 = part_1(&file);
	let p2 = part_2(&file);

	(p1.to_string(), p2.to_string())
}
