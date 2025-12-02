use std::io::{BufRead, Seek};

fn check(nums: &[i32]) -> bool {
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

fn part_1<R: BufRead>(reader: &mut R) -> i32 {
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

	safe_reports
}

fn part_2<R: BufRead>(reader: &mut R) -> i32 {
	let mut safe_reports = 0;

	for line in reader.lines() {
		let line = line.expect("Unable to read line");
		let parts = line.split_whitespace();

		let nums = parts
			.map(|x| x.parse::<i32>().expect("Unable to parse number"))
			.collect::<Vec<i32>>();

		if check(&nums) {
			safe_reports += 1;
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

	safe_reports
}

pub fn solve<R: BufRead + Seek>(mut reader: R) -> (String, String) {
	let p1 = part_1(&mut reader);

	reader.rewind().expect("Unable to rewind");

	let p2 = part_2(&mut reader);

	(p1.to_string(), p2.to_string())
}
