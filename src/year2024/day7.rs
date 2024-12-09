use std::io::{BufRead, Seek};

#[derive(Clone, Copy)]
enum Op {
	Add,
	Multiply,
	Concat,
}

#[inline]
fn concat(a: i64, b: i64) -> i64 {
	a * 10i64.pow(b.ilog10() + 1) + b
}

fn count_numbers(numbers: &Vec<i64>, operations: &[Op]) -> i64 {
	let mut count = numbers[0];

	for i in 1..numbers.len() {
		match operations[i - 1] {
			Op::Add => count += numbers[i],
			Op::Multiply => count *= numbers[i],
			Op::Concat => count = concat(count, numbers[i]),
		}
	}

	count
}

fn count_numbers_binary(numbers: &Vec<i64>, bits: u16) -> i64 {
	let mut count = numbers[0];

	for i in 1..numbers.len() {
		let is_mul = (bits & 2_u16.pow(i as u32 - 1)) > 0;
		if is_mul {
			count *= numbers[i];
		} else {
			count += numbers[i];
		}
	}

	count
}

pub fn solve<R: BufRead + Seek>(reader: R) -> (i64, i64) {
	let mut p1: i64 = 0;
	let mut p2: i64 = 0;

	let mut operations = [Op::Add; 16];

	for line in reader.lines() {
		let line = line.unwrap();
		let (target, numbers): (i64, Vec<i64>) = match line.split_once(':') {
			Some((a, b)) => (
				a.parse().unwrap(),
				b.split_whitespace().map(|n| n.parse().unwrap()).collect(),
			),
			None => panic!("Incorrect line: {}", line),
		};

		// part 1
		let all_mul = 2_u16.pow(numbers.len() as u32 - 1) - 1;
		for i in 0..=all_mul {
			let result = count_numbers_binary(&numbers, i);
			if result == target {
				p1 += target;
				break;
			}
		}

		// part2
		fn recursive(numbers: &Vec<i64>, target: i64, operations: &mut [Op; 16], n: u8) -> bool {
			if n == 0 {
				return count_numbers(numbers, operations) == target;
			}

			if recursive(numbers, target, operations, n - 1) {
				operations[n as usize - 1] = Op::Add;
				return true;
			}

			operations[n as usize - 1] = Op::Multiply;
			if recursive(numbers, target, operations, n - 1) {
				operations[n as usize - 1] = Op::Add;
				return true;
			}

			operations[n as usize - 1] = Op::Concat;
			if recursive(numbers, target, operations, n - 1) {
				operations[n as usize - 1] = Op::Add;
				return true;
			}

			operations[n as usize - 1] = Op::Add;

			return false;
		}

		if recursive(&numbers, target, &mut operations, numbers.len() as u8 - 1) {
			p2 += target;
		}
	}

	(p1, p2)
}
