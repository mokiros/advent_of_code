use regex::Regex;
use std::io::BufRead;

struct Operand(usize);

impl Operand {
	fn combo(&self, a: usize, b: usize, c: usize) -> usize {
		match self.0 {
			0 => 0,
			1 => 1,
			2 => 2,
			3 => 3,
			4 => a,
			5 => b,
			6 => c,
			_ => panic!("Invalid register: {}", self.0),
		}
	}

	fn literal(&self) -> usize {
		self.0 as usize
	}
}

#[derive(Debug)]
enum Opcode {
	ADV, // divide register A by 2^(combo operand), write to register A
	BXL, // bitwise XOR of register B and literal operand, write to register B
	BST, // modulo 8 of combo operand, write to register B
	JNZ, // if A is not 0, jump to literal operand
	BXC, // bitwise XOR of register B and register C, write to register B. Operand is ignored
	OUT, // output modulo 8 of combo operand
	BDV, // same as ADV, but write to register B
	CDV, // same as ADV, but write to register C
}

impl Opcode {
	fn new(op: usize) -> Self {
		match op {
			0 => Self::ADV,
			1 => Self::BXL,
			2 => Self::BST,
			3 => Self::JNZ,
			4 => Self::BXC,
			5 => Self::OUT,
			6 => Self::BDV,
			7 => Self::CDV,
			_ => panic!("Invalid opcode: {}", op),
		}
	}
}

impl std::str::FromStr for Opcode {
	type Err = std::fmt::Error;
	fn from_str(s: &str) -> Result<Self, std::fmt::Error> {
		Ok(Opcode::new(s.parse().unwrap()))
	}
}

fn run(
	instructions: &Vec<usize>,
	mut reg_a: usize,
	mut reg_b: usize,
	mut reg_c: usize,
) -> Vec<usize> {
	let mut ip = 0;
	let mut output: Vec<usize> = Vec::new();

	while ip < instructions.len() {
		let opcode = Opcode::new(instructions[ip]);
		let operand = Operand(instructions[ip + 1]);

		ip += 2;

		match opcode {
			Opcode::ADV => reg_a = reg_a / 2_usize.pow(operand.combo(reg_a, reg_b, reg_c) as u32),
			Opcode::BXL => reg_b = reg_b ^ operand.literal(),
			Opcode::BST => reg_b = operand.combo(reg_a, reg_b, reg_c) % 8,
			Opcode::JNZ => {
				if reg_a != 0 {
					ip = operand.literal() as usize;
					continue;
				}
			}
			Opcode::BXC => reg_b = reg_b ^ reg_c,
			Opcode::OUT => output.push(operand.combo(reg_a, reg_b, reg_c) % 8),
			Opcode::BDV => reg_a = reg_a / 2_usize.pow(operand.combo(reg_a, reg_b, reg_c) as u32),
			Opcode::CDV => reg_c = reg_a / 2_usize.pow(operand.combo(reg_a, reg_b, reg_c) as u32),
		}
	}

	output
}

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	let mut lines = reader.lines();

	let re = Regex::new(r"Register ([ABC]): (\d+)").unwrap();

	let mut reg_a: usize = 0;
	let mut reg_b: usize = 0;
	let mut reg_c: usize = 0;

	for _ in 0..3 {
		let line = lines.next().unwrap().unwrap();
		for cap in re.captures_iter(&line) {
			let register = cap.get(1).unwrap().as_str();
			let value: usize = cap.get(2).unwrap().as_str().parse().unwrap();
			match register {
				"A" => reg_a = value,
				"B" => reg_b = value,
				"C" => reg_c = value,
				_ => panic!("Invalid register: {}", register),
			}
		}
	}

	lines.next();

	let line = lines.next().unwrap().unwrap();
	let instructions: Vec<usize> = line
		.split_whitespace()
		.skip(1)
		.next()
		.unwrap()
		.split(",")
		.map(|s| s.parse().unwrap())
		.collect();

	// part 1
	println!(
		"{}",
		itertools::Itertools::join(
			&mut run(&instructions, reg_a, reg_b, reg_c).into_iter(),
			","
		)
	);

	// part 2
	let mut factors = vec![0; instructions.len()];
	let p2;

	loop {
		let init_a = factors
			.iter()
			.enumerate()
			.map(|(i, f)| 8_usize.pow(i as u32) * f)
			.sum();

		let output = run(&instructions, init_a, reg_b, reg_c);

		if output == instructions {
			p2 = init_a;
			break;
		}

		for (i, op) in instructions.iter().enumerate().rev() {
			if output.len() < i || output[i] != *op {
				factors[i] += 1;
				break;
			}
		}
	}

	(0, p2 as i64)
}
