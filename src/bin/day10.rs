use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day10.txt").unwrap();

	let astroids = file.lines().enumerate().flat_map(|(y, line)| line.chars().enumerate().filter(|&(_, c)| c == '#').map(move |(x, _)| (x as f32, y as f32))).collect::<Vec<_>>();

	let mut max_count = 0;

	let mut part2center = (0.0, 0.0);
	let mut part2hits = Vec::new();

	for a1 in astroids.iter() {
		let mut count = 0;

		let mut hits = Vec::new();
		'x: for a2 in astroids.iter() {
			if a1 == a2 { continue; }

			let slope = (a1.1 - a2.1) / (a1.0 - a2.0);
			let dist1 = (a2.1 - a1.1).abs() + (a2.0 - a1.0).abs();

			for a3 in astroids.iter() {
				if a2 == a3 || a1 == a3 { continue; }
				let dist2 = (a3.1 - a1.1).abs() + (a3.0 - a1.0).abs();

				if a1.0 == a2.0 && a1.0 == a3.0 && ((a1.1 > a2.1 && a1.1 > a3.1) || a1.1 < a2.1 && a1.1 < a3.1) {
					if dist2 < dist1 {
						continue 'x;
					}
					continue;
				}
				if a1.1 == a2.1 && a1.1 == a3.1 && ((a1.0 > a2.0 && a1.0 > a3.0) || a1.0 < a2.0 && a1.0 < a3.0) {
					if dist2 < dist1 {
						continue 'x;
					}
					continue;
				}

				let same_space = (a1.0 - a2.0).is_sign_positive() == (a1.0 - a3.0).is_sign_positive() && (a1.1 - a2.1).is_sign_positive() == (a1.1 - a3.1).is_sign_positive();

				let intersects_line = a3.1 - a1.1 == slope * (a3.0 - a1.0);

				let blocks = same_space && dist2 < dist1 && intersects_line;

				if blocks {
					continue 'x;
				}
			}

			count += 1;
			hits.push(a2.clone());
		}

		if max_count < count {
			max_count = count;
			part2center = *a1;
			part2hits = hits;
		}
	}

	println!("Day 10 part 1: {}", max_count);

	let mut sortlist = part2hits.into_iter().map(|a| {

		let slope = (a.1 - part2center.1) / (a.0 - part2center.0) * 10000.0;

		let quadrant = if a.0 >= part2center.0 && a.1 < part2center.1 {
			1
		} else if a.0 > part2center.0 && a.1 >= part2center.1 {
			2
		} else if a.0 <= part2center.0 && a.1 > part2center.1 {
			3
		} else {
			4
		};

		let slope = if slope.is_infinite() { f32::MIN } else {
			match quadrant {
				1 => slope,
				2 => slope,
				3 => slope,
				4 => slope,
				_ => unreachable!("not possible"),
			}
		};

		(quadrant, slope as i64, (a.0 as usize, a.1 as usize))
	}).collect::<Vec<_>>();
	sortlist.sort();

	let part2 = sortlist[199].2.0 * 100 + sortlist[199].2.1;

	println!("Day 10 part 2: {}", part2);
}