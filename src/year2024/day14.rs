use std::io::BufRead;

pub fn solve<R: BufRead>(reader: R) -> (i64, i64) {
	let re = regex::Regex::new(r"(-?\d+)").unwrap();

	let mut quadrants: [i64; 4] = [0, 0, 0, 0];

	const SECONDS: i64 = 100;
	const EASTER_EGG_SECONDS: i64 = 30000;
	const TILES_X: i64 = 101;
	const TILES_Y: i64 = 103;
	const TILES_X_HALF: i64 = TILES_X / 2;
	const TILES_Y_HALF: i64 = TILES_Y / 2;

	let mut robots: Vec<(i64, i64, i64, i64)> = Vec::new();

	for line in reader.lines() {
		let str = &line.unwrap();
		let mut caps = re
			.captures_iter(str)
			.map(|c| c.get(1).unwrap().as_str().parse::<i64>().unwrap());

		let x = caps.next().unwrap();
		let y = caps.next().unwrap();
		let vx = caps.next().unwrap();
		let vy = caps.next().unwrap();

		robots.push((x, y, vx, vy));

		let end_x = (x + vx * SECONDS).rem_euclid(TILES_X);
		let end_y = (y + vy * SECONDS).rem_euclid(TILES_Y);

		if end_x < TILES_X_HALF && end_y < TILES_Y_HALF {
			quadrants[0] += 1;
		} else if end_x < TILES_X_HALF && end_y > TILES_Y_HALF {
			quadrants[1] += 1;
		} else if end_x > TILES_X_HALF && end_y < TILES_Y_HALF {
			quadrants[2] += 1;
		} else if end_x > TILES_X_HALF && end_y > TILES_Y_HALF {
			quadrants[3] += 1;
		}
	}

	let mut p2 = -1;

	'map_loop: for i in 0..EASTER_EGG_SECONDS {
		let mut map = vec![false; TILES_X as usize * TILES_Y as usize];

		for j in 0..robots.len() {
			let (x, y, vx, vy) = robots[j];

			let end_x = (x + vx * i).rem_euclid(TILES_X);
			let end_y = (y + vy * i).rem_euclid(TILES_Y);

			map[end_y as usize * TILES_X as usize + end_x as usize] = true;
		}

		let mut repeats = 0;
		for is_robot in map {
			if is_robot {
				repeats += 1;
				if repeats >= 10 {
					p2 = i;
					break 'map_loop;
				}
			} else {
				repeats = 0;
			}
		}
	}

	(quadrants.iter().fold(1, |a, b| a * b), p2)
}
