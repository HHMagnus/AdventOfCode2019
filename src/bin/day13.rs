use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day13.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut vec = orr_vec.clone();
    let part1 = day13(&mut vec, false);
	println!("Day 13 part 1: {}", part1);

	let mut vec = orr_vec.clone();
	vec[0] = 2;
    let part2 = day13(&mut vec, true);
	println!("Day 13 part 2: {}", part2);

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

fn day13(arr: &mut Vec<i64>, part2: bool) -> i64 {
	let mut i = 0;
    let mut relative_base = 0;

	let mut outputs = Vec::new();

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			if part2 {
				return outputs.into_iter().as_slice().chunks(3).filter(|x| x[0] == -1 && x[1] == 0).map(|x| x[2]).max().unwrap();
			}
			return outputs.into_iter().as_slice().chunks(3).filter(|x| x[2] == 2).count() as i64;
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
				let paddle = outputs.iter().as_slice().chunks(3).rev().find(|x| x[2] == 3).unwrap()[0];
				let ball = outputs.iter().as_slice().chunks(3).rev().find(|x| x[2] == 4).unwrap()[0];
				let res = if ball < paddle { -1 } else if ball > paddle { 1 } else { 0 }; 
				store(pos, res, arr);
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], arr, relative_base);
				outputs.push(output);
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