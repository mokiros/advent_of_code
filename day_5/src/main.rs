use core::panic;
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader, Seek},
};

fn part_1(reader: &mut BufReader<File>) {
	let mut is_order = true;
	let mut count: u32 = 0;
	let mut ordering = vec![vec![false; 100]; 100];

	'lines: for line in reader.lines() {
		let line = line.expect("Unable to read line");

		if line.is_empty() {
			is_order = false;
			continue;
		}

		if is_order {
			let mut parts = line.split("|");
			let a = parts
				.next()
				.expect("Unable to parse a")
				.parse::<usize>()
				.expect("Unable to parse a");
			let b = parts
				.next()
				.expect("Unable to parse b")
				.parse::<usize>()
				.expect("Unable to parse b");
			ordering[b][a] = true;
			continue;
		}

		let pages = line
			.split(",")
			.map(|x| x.parse::<usize>().expect("Unable to parse number"))
			.collect::<Vec<usize>>();

		for i in 0..pages.len() - 1 {
			let a = pages[i];
			let b = pages[i + 1];
			if ordering[a][b] {
				continue 'lines;
			}
		}

		count += pages[pages.len() / 2] as u32;
	}

	println!("{}", count);
}

fn part_2(reader: &mut BufReader<File>) {
	let mut is_order = true;
	let mut ordering = vec![vec![false; 100]; 100];
	let mut count = 0;

	for line in reader.lines() {
		let line = line.expect("Unable to read line");

		if line.is_empty() {
			is_order = false;
			continue;
		}

		if is_order {
			let mut parts = line.split("|");
			let a = parts
				.next()
				.expect("Unable to parse a")
				.parse::<usize>()
				.expect("Unable to parse a");
			let b = parts
				.next()
				.expect("Unable to parse b")
				.parse::<usize>()
				.expect("Unable to parse b");
			ordering[b][a] = true;
			continue;
		}

		let pages = line
			.split(",")
			.map(|x| x.parse::<usize>().expect("Unable to parse number"))
			.collect::<Vec<usize>>();

		for i in 0..pages.len() - 1 {
			if ordering[pages[i]][pages[i + 1]] {
				for page in pages.iter() {
					let mut sum = 0;
					for n in pages.iter() {
						if *n != *page && ordering[*n][*page] {
							sum += 1;
						}
					}
					if sum == pages.len() / 2 {
						count += page;
					}
				}
				break;
			}
		}
	}

	println!("{}", count);
}

fn main() {
	let file = File::open("input.txt").expect("Unable to open file");
	let mut reader = BufReader::new(file);

	part_1(&mut reader);

	reader.rewind().expect("Unable to rewind");

	part_2(&mut reader);
}
