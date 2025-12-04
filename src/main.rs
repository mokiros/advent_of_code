#![feature(iter_array_chunks)]

use std::{
	fs::File,
	io::{BufRead, BufReader},
	time::Instant,
};

use clap::Parser;

mod util;

mod year2024;

mod year2025;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	year: Option<u16>,

	#[arg(short, long)]
	day: Option<u8>,

	#[arg(short, long)]
	input: Option<String>,

	#[arg(short, long)]
	example_only: Option<bool>,
}

fn run(year: u16, day: u8, input: Option<String>, example_only: bool) -> Result<(), ()> {
	let input_path = input.unwrap_or_else(|| format!("./input/{year}/{day}.txt"));
	let example_input_path = format!("./input/examples/{year}/{day}.txt");
	let example_answer_path = format!("./input/examples/{year}/{day}_answer.txt");

	let solve: fn(BufReader<File>) -> (String, String) = match (year, day) {
		(2024, 1) => year2024::day1::solve,
		(2024, 2) => year2024::day2::solve,
		(2024, 3) => year2024::day3::solve,
		(2024, 4) => year2024::day4::solve,
		(2024, 5) => year2024::day5::solve,
		(2024, 6) => year2024::day6::solve,
		(2024, 7) => year2024::day7::solve,
		(2024, 8) => year2024::day8::solve,
		(2024, 9) => year2024::day9::solve,
		(2024, 10) => year2024::day10::solve,
		(2024, 11) => year2024::day11::solve,
		(2024, 12) => year2024::day12::solve,
		(2024, 13) => year2024::day13::solve,
		(2024, 14) => year2024::day14::solve,
		(2024, 15) => year2024::day15::solve,
		(2024, 16) => year2024::day16::solve,
		(2024, 17) => year2024::day17::solve,
		(2024, 18) => year2024::day18::solve,
		(2024, 19) => year2024::day19::solve,
		(2024, 20) => year2024::day20::solve,
		(2024, 22) => year2024::day22::solve,
		(2024, 23) => year2024::day23::solve,
		(2024, 24) => year2024::day24::solve,
		(2024, 25) => year2024::day25::solve,

		(2025, 1) => year2025::day1::solve,
		(2025, 2) => year2025::day2::solve,
		(2025, 3) => year2025::day3::solve,
		(2025, 4) => year2025::day4::solve,
		_ => return Err(()),
	};

	println!("Year {year}, Day {day}");

	{
		let example_input_file =
			File::open(example_input_path).expect("Unable to open example input file");
		let example_answer_file =
			File::open(example_answer_path).expect("Unable to open example answer file");

		let input_reader = BufReader::new(example_input_file);

		let (part_1, part_2) = solve(input_reader);

		let mut example_reader = BufReader::new(example_answer_file);

		let mut part_1_answer = String::new();
		let mut part_2_answer = String::new();

		example_reader
			.read_line(&mut part_1_answer)
			.expect("Unable to read example part 1 answer");
		example_reader
			.read_line(&mut part_2_answer)
			.expect("Unable to read example part 2 answer");

		let part_1_answer_trimmed = part_1_answer.trim();
		let part_2_answer_trimmed = part_2_answer.trim();

		if !part_1_answer.is_empty() && part_1 != part_1_answer_trimmed {
			println!(
				"Mismatched answers in part 1: Expected {part_1_answer_trimmed}, got {part_1}"
			);
		}
		if !part_2_answer.is_empty() && part_2 != part_2_answer_trimmed {
			println!(
				"Mismatched answers in part 2: Expected {part_2_answer_trimmed}, got {part_2}"
			);
		}
		if example_only {
			return Ok(());
		}
	}

	let file = File::open(input_path).expect("Unable to open file");
	let reader = BufReader::new(file);

	let now = Instant::now();

	let (part_1, part_2) = solve(reader);

	let elapsed = now.elapsed();

	println!("Part 1: {part_1}");
	println!("Part 2: {part_2}");
	println!("Runtime: {elapsed:.2?}");

	Ok(())
}

fn main() {
	let args = Args::parse();

	let years = args.year.map_or(2024..=2025, |year| year..=year);

	let days = args.day.map_or(1..=25, |day| day..=day);

	for year in years {
		for day in days.clone() {
			let result = run(
				year,
				day,
				args.input.clone(),
				args.example_only.unwrap_or(false),
			);

			if result.is_err() {
				return;
			}
		}
	}
}
