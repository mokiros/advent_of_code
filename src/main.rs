use std::{fs::File, io::BufReader, time::Instant};

use clap::Parser;

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

	let file = File::open(input_path).expect("Unable to open file");
	let mut reader = BufReader::new(file);

	let solve: fn(&mut BufReader<File>) -> (i64, i64) = match (year, day) {
		(2024, 1) => year2024::day1::solve,
		(2024, 2) => year2024::day2::solve,
		(2024, 3) => year2024::day3::solve,
		(2024, 4) => year2024::day4::solve,
		(2024, 5) => year2024::day5::solve,
		(2024, 6) => year2024::day6::solve,
		(2024, 7) => year2024::day7::solve,
		_ => return Err(()),
	};

	let now = Instant::now();

	let (part_1, part_2) = solve(&mut reader);

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
		None => 1..=7,
	};

	for year in years {
		for day in days.clone() {
			let _ = run(year, day, args.input.clone());
		}
	}
}