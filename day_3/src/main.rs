use std::fs;

use regex::Regex;

fn part_1(file: &str) {
	let re = Regex::new(r"mul\((\d+),(\d+)\)");

	let mut count = 0;

	// for each match in regex
	for cap in re.unwrap().captures_iter(file) {
		let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
		let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
		count += x * y;
	}

	println!("{}", count);
}

fn part_2(file: &str) {
	let re = Regex::new(r"(mul|do|don't)\(((\d+),(\d+))?\)");

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
			_ => continue,
		}
	}

	println!("{}", count);
}

fn main() {
	let file = fs::read_to_string("input.txt").expect("Unable to read file");

	part_1(file.as_str());

	part_2(file.as_str());
}
