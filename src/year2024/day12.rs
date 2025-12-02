use std::{
	collections::HashSet,
	io::{BufRead, Seek},
};

use crate::util::matrix::Matrix;

fn calculate_area(
	map: &mut Matrix<Option<char>>,
	start_x: isize,
	start_y: isize,
) -> (isize, isize) {
	let plot = map.get(start_x, start_y).flatten().unwrap();
	let mut angles = 0;
	let mut edges_num = 0;
	let mut plots: HashSet<(isize, isize)> = HashSet::new();
	let mut area = 0;

	let mut a: HashSet<(isize, isize)> = HashSet::new();
	a.insert((start_x, start_y));
	let mut b = HashSet::new();

	while !a.is_empty() {
		for (x, y) in a.drain() {
			area += 1;
			map.set(x, y, None);
			plots.insert((x, y));
			let mut previous = (-1, 0);

			let check = |x: isize, y: isize| {
				plots.contains(&(x, y)) || map.get(x, y).flatten() == Some(plot)
			};

			let mut is_previous_plot = check(x - 1, y);
			for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
				let (nx, ny) = (x + dx, y + dy);

				let is_current_plot = check(nx, ny);
				let is_corner_plot = check(nx + previous.0, ny + previous.1);

				if is_current_plot == is_previous_plot
					&& (!is_corner_plot || !is_current_plot)
				{
					angles += 1;
				}

				previous = (dx, dy);
				is_previous_plot = is_current_plot;

				if map.get(nx, ny).flatten() == Some(plot) {
					b.insert((nx, ny));
				} else if !plots.contains(&(nx, ny)) {
					edges_num += 1;
				}
			}
		}

		std::mem::swap(&mut a, &mut b);
		b.clear();
	}

	(area * edges_num, area * angles)
}

pub fn solve<R: BufRead + Seek>(reader: R) -> (String, String) {
	let mut data: Vec<Option<char>> = Vec::new();
	let mut width: u8 = 0;
	let mut height: u8 = 0;

	for line in reader.lines() {
		height += 1;
		width = 0;
		for char in line.unwrap().chars() {
			width += 1;
			data.push(Some(char));
		}
	}

	let mut map = Matrix::new(width, height, data);

	let mut p1 = 0;
	let mut p2 = 0;

	for y in 0..map.height {
		for x in 0..map.width {
			if map.get(x as isize, y as isize).flatten().is_none() {
				continue;
			}
			let (a1, a2) = calculate_area(&mut map, x as isize, y as isize);
			p1 += a1;
			p2 += a2;
		}
	}

	(p1.to_string(), p2.to_string())
}
