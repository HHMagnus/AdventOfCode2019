use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day19.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

	let part1 = (0..50).flat_map(|x| (0..50).map(move |y| (x, y))).map(|xy| day19(orr_vec.clone(), xy)).filter(|&x| x == 1).count();
    println!("Day 19 part 1: {}", part1);
	
	let part2 = part2(orr_vec);
	let part2 = (part2.0 - 99, part2.1 - 99);
	let part2 = part2.0 * 10000 + part2.1;
	println!("Day 19 part 2: {}", part2);
}

fn part2(vec: Vec<i64>) -> (i64, i64) {
	let mut curr = 0;

	let mut map = HashMap::new();

	loop {
		for pos in (0..curr).map(|x| (x, curr)).chain((0..curr+1).map(|y| (curr, y))) {
			let value = day19(vec.clone(), pos);
			let res = value + map.get(&(pos.0, pos.1-1)).unwrap_or(&0) + map.get(&(pos.0-1, pos.1)).unwrap_or(&0) - map.get(&(pos.0-1, pos.1-1)).unwrap_or(&0);
			map.insert(pos, res);

			if pos.0 >= 100 && pos.1 >= 100 {
				let area = res + map.get(&(pos.0 - 100, pos.1 - 100)).unwrap() - map.get(&(pos.0, pos.1 - 100)).unwrap() - map.get(&(pos.0 - 100, pos.1)).unwrap();
				if area == 100*100 {
					return pos;
				}
			}
		}

		curr += 1;
	}
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

fn day19(arr: Vec<i64>, input: (i64, i64)) -> i64 {
	let mut arr = arr;
	let mut i = 0;
    let mut relative_base = 0;
	let mut middle = false;

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			unreachable!("Cannot end");
		}

		match op_code {
			1 => {
				let x1 = load(param1, arr[i+1], &mut arr, relative_base);
				let x2 = load(param2, arr[i+2], &mut arr, relative_base);
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, x1 + x2, &mut arr);
				i += 4;
			},
			2 => {
				let x1 = load(param1, arr[i+1], &mut arr, relative_base);
				let x2 = load(param2, arr[i+2], &mut arr, relative_base);
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, x1 * x2, &mut arr);
				i += 4;
			},
			3 => {
				let pos = pos(param1, arr[i+1], relative_base) as usize;
				let input = if middle { input.1 } else { input.0 };
				middle = true;
				store(pos, input, &mut arr);
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], &mut arr, relative_base);
				i += 2;
				return output;
			},
			5 => {
				let x1 = load(param1, arr[i+1], &mut arr, relative_base);
				let x2 = load(param2, arr[i+2], &mut arr, relative_base);
				if x1 != 0 {
					i = x2 as usize;
				} else {
					i += 3;
				}
			},
			6 => {
				let x1 = load(param1, arr[i+1], &mut arr, relative_base);
				let x2 = load(param2, arr[i+2], &mut arr, relative_base);
				if x1 == 0 {
					i = x2 as usize;
				} else {
					i += 3;
				}
			},
			7 => {
				let x1 = load(param1, arr[i+1], &mut arr, relative_base);
				let x2 = load(param2, arr[i+2], &mut arr, relative_base);
				let result = if x1 < x2 { 1 } else { 0 };
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, result, &mut arr);
				i += 4;

			},
			8 => {
				let x1 = load(param1, arr[i+1], &mut arr, relative_base);
				let x2 = load(param2, arr[i+2], &mut arr, relative_base);
				let result = if x1 == x2 { 1 } else { 0 };
				let pos = pos(param3, arr[i+3], relative_base) as usize;
				store(pos, result, &mut arr);
				i += 4;

			},
            9 => {
                let x1 = load(param1, arr[i+1], &mut arr, relative_base);
                relative_base += x1;
                i += 2;
            }
			x => unreachable!("Unknown opcode {}", x),
		}
	}
}