use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ID(u16);

impl ID {
	fn new(str: &str) -> Self {
		let mut bytes = str.bytes();
		const OFFSET: u16 = b'a' as u16;
		const DIGIT_OFFSET: u16 = b'0' as u16;
		let mut n: u16 = 0;
		for _ in 0..3 {
			let mut d = u16::from(bytes.next().unwrap());
			if d - DIGIT_OFFSET < 10 {
				d -= DIGIT_OFFSET;
			} else {
				d = d - OFFSET + 10;
			}
			n = n * 36 + d;
		}
		Self(n)
	}

	fn get(
		&self,
		values: &mut HashMap<Self, u8>,
		operations: &HashMap<Self, (Operation, Self, Self)>,
	) -> u8 {
		if let Some(value) = values.get(self) {
			*value
		} else {
			let (op, lhs, rhs) = operations.get(self).unwrap();
			let result = op.calc(values, operations, lhs, rhs);
			values.insert(*self, result);
			result
		}
	}
}

impl std::fmt::Display for ID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut n = self.0;
		let mut chars = Vec::new();
		while n > 0 {
			let mut d = n % 36;
			if d >= 10 {
				d += u16::from(b'a') - 10;
			} else {
				d += u16::from(b'0');
			}
			chars.push(d as u8);
			n /= 36;
		}
		chars.reverse();
		write!(f, "{}", String::from_utf8(chars).unwrap())
	}
}

enum Operation {
	AND,
	OR,
	XOR,
}

impl Operation {
	fn calc(
		&self,
		values: &mut HashMap<ID, u8>,
		operations: &HashMap<ID, (Self, ID, ID)>,
		lhs: &ID,
		rhs: &ID,
	) -> u8 {
		let v1 = lhs.get(values, operations);
		let v2 = rhs.get(values, operations);

		match self {
			Self::AND => v1 & v2,
			Self::OR => v1 | v2,
			Self::XOR => v1 ^ v2,
		}
	}
}

impl From<&str> for Operation {
	fn from(value: &str) -> Self {
		match value {
			"AND" => Self::AND,
			"OR" => Self::OR,
			"XOR" => Self::XOR,
			_ => panic!("Invalid operation: {value}"),
		}
	}
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let mut values: HashMap<ID, u8> = HashMap::new();
	let mut operations: HashMap<ID, (Operation, ID, ID)> = HashMap::new();

	let mut outputs: Vec<ID> = Vec::new();

	let mut reading_operations = false;

	for line in reader.lines() {
		let line = line.unwrap();

		if line.is_empty() {
			reading_operations = true;
			continue;
		}

		let mut split = line.split_whitespace();
		if reading_operations {
			let lhs = ID::new(split.next().unwrap());
			let op = Operation::from(split.next().unwrap());
			let rhs = ID::new(split.next().unwrap());
			split.next();
			let result = ID::new(split.next().unwrap());

			if result >= ID::new("z00") {
				outputs.push(result);
			}

			operations.insert(result, (op, lhs, rhs));
		} else {
			let id = ID::new(split.next().unwrap());
			let num: u8 = split.next().unwrap().parse().unwrap();
			values.insert(id, num);
		}
	}

	outputs.sort();

	let p1: i64 = outputs.iter().rev().fold(0, |p1, id| {
		(p1 << 1) | i64::from(id.get(&mut values, &operations))
	});

	(p1.to_string(), 0.to_string())
}
