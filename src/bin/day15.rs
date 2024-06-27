use std::{collections::{HashSet, VecDeque}, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day15.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();
	visited.insert((0,0));
	queue.push_back((orr_vec, 0, 0, (0,0), 1));

	let mut open_spaces = HashSet::new();
	open_spaces.insert((0,0));
	let mut part1 = (0, 0);

	while let Some(next) = queue.pop_front() {
		let changes = [
			(1, (next.3.0, next.3.1 - 1)),
			(2, (next.3.0, next.3.1 + 1)),
			(3, (next.3.0 - 1, next.3.1)),
			(4, (next.3.0 + 1, next.3.1)),
		];

		for change in changes {
			if visited.contains(&change.1) { continue; }
			visited.insert(change.1);

			let (vec, i, rb, output) = day15(next.0.clone(), next.1, next.2, change.0);

			if output == 2 {
				part1 = change.1;
				println!("Day 15 part 1: {}", next.4);
				continue;
			}
			else if output == 0 {
				continue;
			}

			queue.push_back((vec, i, rb, change.1, next.4+1));
			open_spaces.insert(change.1);
		}
	}

	let mut i = 0;

	let mut positions = vec![part1];

	while !open_spaces.is_empty() {
		let xs = positions;
		positions = Vec::new();
		i += 1;
		for x in xs {
			let change = [
				(x.0, x.1+1),
				(x.0, x.1-1),
				(x.0+1, x.1),
				(x.0-1, x.1),
			];

			for c in change {
				if open_spaces.contains(&c) {
					open_spaces.remove(&c);
					positions.push(c);
				}
			}
		}
	}

	println!("Day 15 part 2: {:?}", i);
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

fn day15(mut arr: Vec<i64>, mut i: usize, mut relative_base: i64, input: i64) -> (Vec<i64>, usize, i64, i64){
	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			unreachable!("Not ending");
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
				store(pos, input, &mut arr);
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], &mut arr, relative_base);
				i += 2;
				return (arr, i, relative_base, output);
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