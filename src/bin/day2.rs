use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day2.txt").unwrap();

	let orr = file.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let mut arr = orr.clone();

	arr[1] = 12;
	arr[2] = 2;

	day2(&mut arr);

	println!("Day 2 part 1: {}", arr[0]);

	for noun in 0..=99 {
		for verb in 0..=99 {
			let mut arr = orr.clone();
			arr[1] = noun;
			arr[2] = verb;
			day2(&mut arr);
			if arr[0] == 19690720 {
				println!("Day 2 part 2: {}", 100 * noun + verb);
				return;
			}
		}
	}
}

fn day2(arr: &mut Vec<usize>) {
	let mut i = 0;

	loop {
		let op_code = arr[i];
		if op_code == 99 {
			break;
		}

		let pos1 = arr[i+1];
		let pos2 = arr[i+2];
		let pos3 = arr[i+3];

		arr[pos3] = match op_code {
			1 => arr[pos1] + arr[pos2],
			2 => arr[pos1] * arr[pos2],
			x => unreachable!("Unknown opcode {}", x)
		};

		i += 4;
	}
}