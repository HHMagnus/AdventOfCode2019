use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day5.txt").unwrap();

    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut vec = orr_vec.clone();
	println!("Day 5 part 1:");
	day5(&mut vec, 1);

    let mut vec = orr_vec;

	println!("Day 5 part 2:");
	day5(&mut vec, 5);
}

fn load(param: i64, arg: i64, arr: &mut Vec<i64>) -> i64 {
	if param == 0 {
		arr[arg as usize]
	} else {
		arg
	}
}

fn day5(arr: &mut Vec<i64>, input: i64) {
	let mut i = 0;

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000;
		if op_code == 99 {
			break;
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
				arr[pos] = input;
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], arr);
				println!("{}", output);
				i += 2;
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