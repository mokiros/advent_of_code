use std::{
	collections::HashMap,
	io::{BufRead, Seek},
};

fn part_1<R: BufRead>(reader: &mut R) -> i32 {
	let mut x_values = Vec::new();
	let mut y_values = Vec::new();

	for line in reader.lines() {
		let line = line.expect("Unable to read line");
		let mut parts = line.split_whitespace();
		let x = parts
			.next()
			.expect("Unable to parse x")
			.parse::<i32>()
			.expect("Unable to parse x");
		let y = parts
			.next()
			.expect("Unable to parse y")
			.parse::<i32>()
			.expect("Unable to parse y");
		x_values.push(x);
		y_values.push(y);
	}

	x_values.sort();
	y_values.sort();

	let mut sum = 0;

	for i in 0..x_values.len() {
		let x = x_values[i];
		let y = y_values[i];
		sum += (x - y).abs();
	}

	return sum;
}

fn part_2<R: BufRead>(reader: &mut R) -> i32 {
	let mut values: Vec<i32> = Vec::new();
	let mut frequency: HashMap<i32, i32> = HashMap::new();

	for line in reader.lines() {
		let line = line.expect("Unable to read line");
		let mut parts = line.split_whitespace();
		let x = parts
			.next()
			.expect("Unable to parse x")
			.parse::<i32>()
			.expect("Unable to parse x");
		let y = parts
			.next()
			.expect("Unable to parse y")
			.parse::<i32>()
			.expect("Unable to parse y");
		values.push(x);
		frequency.insert(y, frequency.get(&y).unwrap_or(&0) + 1);
	}

	let mut sum = 0;

	for i in 0..values.len() {
		let x = values[i];
		let y = frequency.get(&x).unwrap_or(&0);
		sum += x * y;
	}

	return sum;
}

pub fn solve<R: BufRead + Seek>(reader: &mut R) -> (i64, i64) {
	let p1 = part_1(reader);

	reader.rewind().expect("Unable to rewind");

	let p2 = part_2(reader);

	(p1 as i64, p2 as i64)
}
