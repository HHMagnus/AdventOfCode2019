use std::{collections::VecDeque, fs::read_to_string, io};

fn main() {
    let file = read_to_string("input/day25.txt").unwrap();
    let mut vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    day25(&mut vec);
	// The day is a text based adventure.
	// I explored the map and discovered that the security door would open and show the result by having the following items in my inventory:
	// coin, mug, hypercube, astrolabe
	// Following directions: west, take mug, east, east, take coin, north, north, take hypercube, south, south, south, west, take astrolabe, north, east, north, east
	// This was done by exploring the rooms and writing it down on a piece of paper
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

fn day25(arr: &mut Vec<i64>) {
	let mut stdin = io::stdin();
	let mut i = 0;
    let mut relative_base = 0;
	let mut outgoing = Vec::new();
	let mut input = VecDeque::new();

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			let message = outgoing.iter().collect::<String>();
			for line in message.lines() {
				println!("{}", line);
			}
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
				if !outgoing.is_empty() {
					let message = outgoing.iter().collect::<String>();
					for line in message.lines() {
						println!("{}", line);
					}
					outgoing.clear();
				}
				if input.is_empty() {
					let take_input = &mut String::new();
					let _ = stdin.read_line(take_input);
					input = take_input.chars().map(|x| x as u32 as i64).collect();
					let _ = input.pop_back();
					let _ = input.pop_back();
					input.push_back('\n' as u32 as i64);
					println!("Gave command: {:?}", take_input);
				}
				store(pos, input.pop_front().unwrap(), arr);
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], arr, relative_base);
				outgoing.push(char::from_u32(output as u32).unwrap());
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