use std::io::{BufRead, Seek};

fn part_1<R: BufRead>(reader: &mut R) -> i32 {
	todo!("Not implemented yet");
}

fn part_2<R: BufRead>(reader: &mut R) -> i32 {
	todo!("Not implemented yet");
}

pub fn solve<R: BufRead + Seek>(reader: &mut R) -> (i64, i64) {
	let p1 = part_1(reader);

	reader.rewind().expect("Unable to rewind");

	let p2 = part_2(reader);

	(p1 as i64, p2 as i64)
}
