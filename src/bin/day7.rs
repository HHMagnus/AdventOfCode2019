use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day7.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

	let mut part1 = 0;

	for i1 in 0..=4 {
		let mut vec = orr_vec.clone();
		let output = day7(0, &mut vec, [i1, 0]).unwrap().0;
		for i2 in 0..=4 {
			if i1 == i2 { continue }
			let mut vec = orr_vec.clone();
			let output = day7(0, &mut vec, [i2, output]).unwrap().0;
			for i3 in 0..=4 {
				if i1 == i3 || i2 == i3 { continue }
				let mut vec = orr_vec.clone();
				let output = day7(0, &mut vec, [i3, output]).unwrap().0;
				for i4 in 0..=4 {
					if i1 == i4 || i2 == i4 || i3 == i4 { continue }
					let mut vec = orr_vec.clone();
					let output = day7(0, &mut vec, [i4, output]).unwrap().0;
					for i5 in 0..=4 {
						if i1 == i5 || i2 == i5 || i3 == i5 || i4 == i5 { continue }
						let mut vec = orr_vec.clone();
						let output = day7(0, &mut vec, [i5, output]).unwrap().0;
						if output > part1 {
							part1 = output;
						}
					}
				}
			}
		}
	}
	println!("Day 5 part 1: {}", part1);

	let mut part2 = 0;

	for i1 in 5..=9 {
		for i2 in 5..=9 {
			if i1 == i2 { continue }
			for i3 in 5..=9 {
				if i1 == i3 || i2 == i3 { continue }
				for i4 in 5..=9 {
					if i1 == i4 || i2 == i4 || i3 == i4 { continue }
					for i5 in 5..=9 {
						if i1 == i5 || i2 == i5 || i3 == i5 || i4 == i5 { continue }
						let output = run_part2(&orr_vec, [i1, i2, i3, i4, i5]);
						if output > part2 {
							part2 = output;
						}
					}
				}
			}
		}
	}

	println!("Day 5 part 2: {}", part2);
}

fn run_part2(arr: &Vec<i64>, phase_settings: [i64; 5]) -> i64 {
	let mut vec1 = arr.clone();
	let mut c1 = 0;
	let mut vec2 = arr.clone();
	let mut c2 = 0;
	let mut vec3 = arr.clone();
	let mut c3 = 0;
	let mut vec4 = arr.clone();
	let mut c4 = 0;
	let mut vec5 = arr.clone();
	let mut c5 = 0;
	
	let mut start = 0;

	let mut first = true;

	'l: loop {
		let output = day7(c1, &mut vec1, if first { [phase_settings[0], start] } else { [start, 0] });
		if output.is_none() {
			break 'l;
		}
		let output = output.unwrap();
		c1 = output.1;
		let output = output.0;
		let output = day7(c2, &mut vec2, if first { [phase_settings[1], output] } else { [output, 0] });
		if output.is_none() {
			break 'l;
		}
		let output = output.unwrap();
		c2 = output.1;
		let output = output.0;
		let output = day7(c3, &mut vec3, if first { [phase_settings[2], output] } else { [output, 0] });
		if output.is_none() {
			break 'l;
		}
		let output = output.unwrap();
		c3 = output.1;
		let output = output.0;
		let output = day7(c4, &mut vec4, if first { [phase_settings[3], output] } else { [output, 0] });
		if output.is_none() {
			break 'l;
		}
		let output = output.unwrap();
		c4 = output.1;
		let output = output.0;
		let output = day7(c5, &mut vec5, if first { [phase_settings[4], output] } else { [output, 0] });
		if output.is_none() {
			break 'l;
		}
		let output = output.unwrap();
		c5 = output.1;
		let output = output.0;
		start = output;
		first = false;
	}

	start
}

fn load(param: i64, arg: i64, arr: &mut Vec<i64>) -> i64 {
	if param == 0 {
		arr[arg as usize]
	} else {
		arg
	}
}

fn day7(mut i: usize, arr: &mut Vec<i64>, phase_settings: [i64;2]) -> Option<(i64, usize)> {
	let mut curr_input = 0;

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000;
		if op_code == 99 {
			return None;
		}

		match op_code {
			1 => {
				let x1 = load(param1, arr[i+1], arr);
				let x2 = load(param2, arr[i+2], arr);
				let pos = arr[i+3] as usize;
				arr[pos] = x1 + x2;
				i += 4;
			},
			2 => {
				let x1 = load(param1, arr[i+1], arr);
				let x2 = load(param2, arr[i+2], arr);
				let pos = arr[i+3] as usize;
				arr[pos] = x1 * x2;
				i += 4;
			},
			3 => {
				let pos = arr[i+1] as usize;
				arr[pos] = phase_settings[curr_input];
				curr_input += 1;
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], arr);
				i += 2;
				return Some((output, i));
			},
			5 => {
				let x1 = load(param1, arr[i+1], arr);
				let x2 = load(param2, arr[i+2], arr);
				if x1 != 0 {
					i = x2 as usize;
				} else {
					i += 3;
				}
			},
			6 => {
				let x1 = load(param1, arr[i+1], arr);
				let x2 = load(param2, arr[i+2], arr);
				if x1 == 0 {
					i = x2 as usize;
				} else {
					i += 3;
				}
			},
			7 => {
				let x1 = load(param1, arr[i+1], arr);
				let x2 = load(param2, arr[i+2], arr);
				let store = if x1 < x2 { 1 } else { 0 };
				let pos = arr[i+3] as usize;
				arr[pos] = store;
				i += 4;

			},
			8 => {
				let x1 = load(param1, arr[i+1], arr);
				let x2 = load(param2, arr[i+2], arr);
				let store = if x1 == x2 { 1 } else { 0 };
				let pos = arr[i+3] as usize;
				arr[pos] = store;
				i += 4;

			},
			x => unreachable!("Unknown opcode {}", x),
		}
	}
}