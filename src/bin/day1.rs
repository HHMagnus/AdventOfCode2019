use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day1.txt").unwrap();

	let lines = file.lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

	let part1 = lines.iter().map(|x| x / 3 - 2).sum::<i32>();
	println!("Day 1 part 1: {}", part1);

	let part2 = lines.into_iter().map(mass).sum::<i32>();
	println!("Day 1 part 2: {}", part2);
}

fn mass(x: i32) -> i32 {
	let nmass = x / 3 - 2;
	if nmass <= 0 {
		return 0;
	}
	nmass + mass(nmass)
}
