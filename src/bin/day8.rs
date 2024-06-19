use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day8.txt").unwrap();

	let digits = file.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>();
	let mut min_0 = usize::MAX;
	let mut part1 = 0;
	let mut layers = Vec::new();
	for i in (0..(digits.len()/(25*6))).map(|x| x * 25*6) {
		let relevant = digits[i .. (i+25*6)].to_vec();
		let tmin_0 = relevant.iter().filter(|&&x| x == 0).count();
		if min_0 > tmin_0 {
			min_0 = tmin_0;
			let one = relevant.iter().filter(|&&x| x == 1).count();
			let two = relevant.iter().filter(|&&x| x == 2).count();
			part1 = one*two;
		}
		layers.push(relevant);
	}

	println!("Day 8 part 1: {}", part1);

	println!("Day 8 part 2:");
	for y in 0..6 {
		for x in 0..25 {
			let coord = x + (25 * y);
			for layer in &layers {
				if layer[coord] == 2 {
					continue;
				}

				print!("{}", if layer[coord] == 0 { "." } else { "#" });
				break;
			}
		}
		println!("");
	}
}