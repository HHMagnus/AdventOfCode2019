use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day7.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

	let mut part1 = 0;

	for i1 in 0..=4 {
		let mut vec = orr_vec.clone();
		let output = day7(&mut vec, [i1, 0]);
		for i2 in 0..=4 {
			if i1 == i2 { continue }
			let mut vec = orr_vec.clone();
			let output = day7(&mut vec, [i2, output]);
			for i3 in 0..=4 {
				if i1 == i3 || i2 == i3 { continue }
				let mut vec = orr_vec.clone();
				let output = day7(&mut vec, [i3, output]);
				for i4 in 0..=4 {
					if i1 == i4 || i2 == i4 || i3 == i4 { continue }
					let mut vec = orr_vec.clone();
					let output = day7(&mut vec, [i4, output]);
					for i5 in 0..=4 {
						if i1 == i5 || i2 == i5 || i3 == i5 || i4 == i5 { continue }
						let mut vec = orr_vec.clone();
						let output = day7(&mut vec, [i5, output]);
						if output > part1 {
							part1 = output;
						}
					}
				}
			}
		}
	}
	println!("Day 5 part 1: {}", part1);
}

fn load(param: i64, arg: i64, arr: &mut Vec<i64>) -> i64 {
	if param == 0 {
		arr[arg as usize]
	} else {
		arg
	}
}

fn day7(arr: &mut Vec<i64>, phase_settings: [i64; 2]) -> i64 {
	let mut i = 0;

	let mut curr_input = 0;

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000;
		if op_code == 99 {
			unreachable!("ended");
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
				return output;
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