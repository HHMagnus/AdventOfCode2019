use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day8.txt").unwrap();

	let digits = file.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>();
	let mut min_0 = usize::MAX;
	let mut part1 = 0;
	for i in (0..(digits.len()/(25*6))).map(|x| x * 25*6) {
		let relevant = digits[i .. (i+25*6)].to_vec();
		println!("{:?}", relevant.len());
		let tmin_0 = relevant.iter().filter(|&&x| x == 0).count();
		if min_0 > tmin_0 {
			min_0 = tmin_0;
			let one = relevant.iter().filter(|&&x| x == 1).count();
			let two = relevant.iter().filter(|&&x| x == 2).count();
			part1 = one*two;
		}
	}

	println!("Day 8 part 1: {}", part1);
}