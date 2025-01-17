use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day9.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    println!("Day 9 part 1:");
    let mut vec = orr_vec.clone();
    day9(&mut vec, 1);

	println!("Day 9 part 2:");
    let mut vec = orr_vec.clone();
    day9(&mut vec, 2);
}

fn pos(param: i64, arg: i64, relative_base: i64) -> usize {
	if param == 0 {
		arg as usize
	} else if param == 2 {
        (relative_base + arg) as usize
    } else {
        unreachable!("No pos from param: {}", param);
    } 
}


fn load(param: i64, arg: i64, arr: &mut Vec<i64>, relative_base: i64) -> i64 {
	if param == 1 {
		return arg;
	}

	let index = pos(param, arg, relative_base);

	while index >= arr.len() {
		arr.push(0);
	}

	arr[index]
}

fn store(pos: usize, result: i64, arr: &mut Vec<i64>) {
	while pos >= arr.len() {
		arr.push(0);
	}
	arr[pos] = result;
}

fn day9(arr: &mut Vec<i64>, input: i64) {
	let mut i = 0;
    let mut relative_base = 0;

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			break;
		}

		match op_code {
			1 => {
				let x1 = load(param1, arr[i+1], arr, relative_base);
				let x2 = load(param2, arr[i+2], arr, relative_base);
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, x1 + x2, arr);
				i += 4;
			},
			2 => {
				let x1 = load(param1, arr[i+1], arr, relative_base);
				let x2 = load(param2, arr[i+2], arr, relative_base);
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, x1 * x2, arr);
				i += 4;
			},
			3 => {
				let pos = pos(param1, arr[i+1], relative_base) as usize;
				store(pos, input, arr);
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], arr, relative_base);
				println!("{}", output);
				i += 2;
			},
			5 => {
				let x1 = load(param1, arr[i+1], arr, relative_base);
				let x2 = load(param2, arr[i+2], arr, relative_base);
				if x1 != 0 {
					i = x2 as usize;
				} else {
					i += 3;
				}
			},
			6 => {
				let x1 = load(param1, arr[i+1], arr, relative_base);
				let x2 = load(param2, arr[i+2], arr, relative_base);
				if x1 == 0 {
					i = x2 as usize;
				} else {
					i += 3;
				}
			},
			7 => {
				let x1 = load(param1, arr[i+1], arr, relative_base);
				let x2 = load(param2, arr[i+2], arr, relative_base);
				let result = if x1 < x2 { 1 } else { 0 };
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, result, arr);
				i += 4;

			},
			8 => {
				let x1 = load(param1, arr[i+1], arr, relative_base);
				let x2 = load(param2, arr[i+2], arr, relative_base);
				let result = if x1 == x2 { 1 } else { 0 };
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, result, arr);
				i += 4;

			},
            9 => {
                let x1 = load(param1, arr[i+1], arr, relative_base);
                relative_base += x1;
                i += 2;
            }
			x => unreachable!("Unknown opcode {}", x),
		}
	}
}