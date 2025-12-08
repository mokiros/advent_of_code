use std::io::{BufRead, Seek};

fn read_ranges<R: BufRead>(reader: &mut R) -> Vec<std::ops::RangeInclusive<u64>> {
	let mut buf = vec![];

	let mut ranges = vec![];

	loop {
		let len = reader.read_until(b'-', &mut buf).unwrap();
		if len == 0 {
			break;
		}
		if buf.last() == Some(&b'-') {
			buf.pop();
		}

		let start = str::from_utf8(&buf).unwrap().parse::<u64>().unwrap();
		buf.clear();

		let len = reader.read_until(b',', &mut buf).unwrap();
		assert!(len != 0, "Reached EOF when reading end number");
		if buf.last() == Some(&b',') || buf.last() == Some(&b'\n') {
			buf.pop();
		}

		let end = str::from_utf8(&buf).unwrap().parse::<u64>().unwrap();
		buf.clear();

		ranges.push(start..=end);
	}

	ranges
}

fn is_valid(n: u64) -> (bool, bool) {
	let digits = n.ilog10() + 1;
	let digits_half = digits / 2;
	'outer: for sequence_length in (1..=digits_half).rev() {
		if !digits.is_multiple_of(sequence_length) {
			continue;
		}
		let modulo = 10_u64.pow(sequence_length);
		let first_part = n % modulo;
		for sequence_num in (sequence_length..digits).step_by(sequence_length.try_into().unwrap()) {
			let second_part = (n / (10_u64.pow(sequence_num))) % (10_u64.pow(sequence_length));
			if first_part != second_part {
				continue 'outer;
			}
		}
		let is_half = digits.is_multiple_of(2) && sequence_length == digits_half;
		return (is_half, true);
	}
	(false, false)
}

pub fn solve(buffer: &[u8]) -> (String, String) {
	let mut reader = std::io::Cursor::new(buffer);
	let mut sum1: u64 = 0;
	let mut sum2: u64 = 0;

	let ranges = read_ranges(&mut reader);

	for range in ranges {
		for n in range {
			let (valid1, valid2) = is_valid(n);
			if valid2 {
				sum2 += n;
				if valid1 {
					sum1 += n;
				}
			}
		}
	}

	(sum1.to_string(), sum2.to_string())
}
