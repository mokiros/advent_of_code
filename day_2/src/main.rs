use std::{
	fs::File,
	io::{BufRead, BufReader, Seek},
};

fn check(nums: &Vec<i32>) -> bool {
	if nums.len() <= 1 {
		return true;
	}

	let mut prev = nums[0];
	if prev == nums[1] {
		return false;
	}
	let increasing = nums[1] > prev;

	for num in nums.iter().skip(1) {
		let diff = num - prev;
		if diff == 0 || diff.abs() > 3 {
			return false;
		}
		if (diff > 0) != increasing {
			return false;
		}
		prev = *num;
	}

	true
}

fn part_1(reader: &mut BufReader<File>) {
	let mut safe_reports = 0;

	for line in reader.lines() {
		let line = line.expect("Unable to read line");
		let parts = line.split_whitespace();

		let nums = parts
			.map(|x| x.parse::<i32>().expect("Unable to parse number"))
			.collect::<Vec<i32>>();

		if check(&nums) {
			safe_reports += 1;
		}
	}

	println!("{}", safe_reports);
}

fn part_2(reader: &mut BufReader<File>) {
	let mut safe_reports = 0;

	for line in reader.lines() {
		let line = line.expect("Unable to read line");
		let parts = line.split_whitespace();

		let nums = parts
			.map(|x| x.parse::<i32>().expect("Unable to parse number"))
			.collect::<Vec<i32>>();

		if check(&nums) {
			safe_reports += 1
		} else {
			for i in 0..nums.len() {
				let mut nums = nums.clone();
				nums.remove(i);
				if check(&nums) {
					safe_reports += 1;
					break;
				}
			}
		}
	}

	println!("{}", safe_reports);
}

fn main() {
	let file = File::open("input.txt").expect("Unable to open file");
	let mut reader = BufReader::new(file);

	part_1(&mut reader);

	reader.rewind().expect("Unable to rewind");

	part_2(&mut reader);
}
