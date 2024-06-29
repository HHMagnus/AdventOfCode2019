use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day17.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let outputs = day17(orr_vec.clone(), vec![]);
	let outputs = outputs.into_iter().map(|x| char::from_u32(x as u32).unwrap()).collect::<String>();
	let outputs = outputs.lines().collect::<Vec<_>>();
	
	let set = outputs.iter().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter_map(move |(x, c)| if c == '#' { Some((x as i32,y as i32)) } else { None })).collect::<HashSet<_>>();
	let maxx = set.iter().map(|&(x, _)| x).max().unwrap();
	let maxy = set.iter().map(|&(_, y)| y).max().unwrap();

	let mut part1 = 0;
	for y in 0..=maxy {
		for x in 0..=maxx {
			if set.contains(&(x, y)) && set.contains(&(x, y-1)) && set.contains(&(x+1, y)) && set.contains(&(x, y+1)) && set.contains(&(x-1, y)) {
				part1 += x*y;
			}
		}
	}
	
	println!("Day 17 part 1: {}", part1);

	for output in outputs {
		println!("{}", output);
	}

	// Part 2 is solved manually:
	// L8 R10 L10 R10 L8 L8 L10 L8 R10 L10 L4 L6 L8 L8 R10 L8 L8 L10 L4 L6 L8 L8 L8 R10 L10 L4 L6 L8 L8 R10 L8 L8 L10 L4 L6 L8 L8
	// A  B   C   B   A  A  C   A  B   C   D  E  A  A  B   A  A  C   D  E  A  A  A  B   C   D  E  A  A  B   A  A  C   D  E  A  A
	// ABC BAAC ABC DEAA BAAC DEAA ABC DEAA BAAC DEAA
	// MR = A B A C B C A C B C
	// A  = L,8,R,10,L,10
	// B  = R,10,L,8,L,8,L,10
	// C  = L,4,L,6,L,8,L,8

	let mut part2vec = orr_vec.clone();
	part2vec[0] = 2;

	let inputs = "A,B,A,C,B,C,A,C,B,C\nL,8,R,10,L,10\nR,10,L,8,L,8,L,10\nL,4,L,6,L,8,L,8\nn\n".chars().map(|x| x as u32 as i64).collect::<Vec<i64>>();
    let outputs = day17(part2vec, inputs);
	println!("Day 17 part 2: {}", outputs.last().unwrap());
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

fn day17(mut arr: Vec<i64>, input: Vec<i64>) -> Vec<i64> {
	let mut i = 0;
    let mut relative_base = 0;

	let mut outputs = Vec::new();
	let mut input_i = 0;

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			return outputs;
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
				store(pos, input[input_i], &mut arr);
				input_i += 1;
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], &mut arr, relative_base);
				outputs.push(output);
				i += 2;
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