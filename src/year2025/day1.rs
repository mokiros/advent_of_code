use std::io::{BufRead, Seek};

pub fn solve<R: BufRead + Seek>(reader: R) -> (String, String) {
	let mut rot = 50;
	let mut zeroes = 0;
	let mut wraps = 0;

	for line in reader.lines() {
		let line = line.expect("Unable to read line");
		let mut chars = line.chars();
		let letter = chars.next().expect("Line is empty");
		let num = chars.as_str().parse::<i32>().expect("Invalid number");
		let num = match letter {
			'L' => {
				wraps += num / 100;
				if rot != 0 && num % 100 >= rot {
					wraps += 1;
				}
				-num
			}
			'R' => {
				wraps += (rot + num) / 100;
				num
			}
			letter => panic!("Invalid letter: {letter}"),
		};
		rot = (rot + num).rem_euclid(100);
		if rot == 0 {
			zeroes += 1;
		}
	}

	(zeroes.to_string(), wraps.to_string())
}
