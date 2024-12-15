use std::{fs::File, io::BufReader, time::Instant};

use clap::Parser;

mod util;
mod year2024;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	year: Option<u16>,

	#[arg(short, long)]
	day: Option<u8>,

	#[arg(short, long)]
	input: Option<String>,
}

fn run(year: u16, day: u8, input: Option<String>) -> Result<(), ()> {
	let input_path = input.unwrap_or(format!("./input/{}/{}.txt", year, day));

	let solve: fn(BufReader<File>) -> (i64, i64) = match (year, day) {
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
		_ => return Err(()),
	};

	let file = File::open(input_path).expect("Unable to open file");
	let reader = BufReader::new(file);

	let now = Instant::now();

	let (part_1, part_2) = solve(reader);

	let elapsed = now.elapsed();

	println!("Year {}, Day {}", year, day);
	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2);
	println!("Runtime: {:.2?}", elapsed);

	Ok(())
}

fn main() {
	let args = Args::parse();

	let years = match args.year {
		Some(year) => year..=year,
		None => 2024..=2024,
	};

	let days = match args.day {
		Some(day) => day..=day,
		None => 1..=25,
	};

	for year in years {
		for day in days.clone() {
			let result = run(year, day, args.input.clone());

			if result.is_err() {
				return;
			}
		}
	}
}
