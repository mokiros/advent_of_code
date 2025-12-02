use std::{
	collections::HashMap,
	io::{BufRead, Seek},
	thread,
};

// keeping this just incase I get a supercomputer to actually compute this
#[allow(dead_code)]
fn naive<R: BufRead + Seek>(mut reader: R) -> (String, String) {
	let mut str = String::new();
	reader.read_to_string(&mut str).unwrap();
	let mut stones = str
		.split_whitespace()
		.map(|s| s.parse::<u64>().unwrap())
		.collect::<Vec<u64>>();
	let mut len = stones.len();

	let mut p1 = 0;

	for i in 0..75 {
		println!("Iteration {i}, count: {len}");
		if i == 25 {
			p1 = len;
		}
		for k in 0..len {
			let n = stones[k];
			if n == 0 {
				stones[k] = 1;
			} else {
				let log = n.ilog10();
				if log % 2 == 0 {
					stones[k] = n * 2024;
				} else {
					let l = 10_u64.pow(log.div_ceil(2));
					stones[k] = n / l;
					stones.push(n % l);
					len += 1;
				}
			}
		}
		thread::sleep(std::time::Duration::from_millis(100));
	}

	(p1.to_string(), len.to_string())
}

pub fn solve<R: BufRead + Seek>(mut reader: R) -> (String, String) {
	let mut str = String::new();
	reader.read_to_string(&mut str).unwrap();

	let mut stones: HashMap<u64, i64> = str
		.split_whitespace()
		.map(|s| s.parse().unwrap())
		.map(|n| (n, 1))
		.collect();
	let mut new_stones: HashMap<u64, i64> = HashMap::with_capacity(stones.len());

	let mut p1 = 0;

	for i in 0..75 {
		if i == 25 {
			p1 = stones.values().sum();
		}

		for (n, count) in &stones {
			if *n == 0 {
				*new_stones.entry(1).or_insert(0) += count;
			} else {
				let log = n.ilog10();
				if log % 2 == 0 {
					*new_stones.entry(n * 2024).or_insert(0) += count;
				} else {
					let l = 10_u64.pow(log.div_ceil(2));
					*new_stones.entry(n / l).or_insert(0) += count;
					*new_stones.entry(n % l).or_insert(0) += count;
				}
			}
		}

		std::mem::swap(&mut stones, &mut new_stones);
		new_stones.clear();
	}

	(p1.to_string(), stones.values().sum::<i64>().to_string())
}
