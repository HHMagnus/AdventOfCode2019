use std::{collections::HashSet, fs::read_to_string};

enum Dir {
	Up,
	Right,
	Down,
	Left,
}

fn main() {
    let file = read_to_string("input/day11.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut vec = orr_vec.clone();
    let part1 = day11(&mut vec, false).len();
    println!("Day 11 part 1: {}", part1);

    let mut vec = orr_vec.clone();
	let whites = day11(&mut vec, true);
	let miny = *whites.iter().map(|(_, y)| y).min().unwrap();
	let maxy = *whites.iter().map(|(_, y)| y).max().unwrap();
	let minx = *whites.iter().map(|(x, _)| x).min().unwrap();
	let maxx = *whites.iter().map(|(x, _)| x).max().unwrap();

    println!("Day 11 part 2:");
	for y in miny..=maxy {
		for x in minx..maxx {
			if whites.contains(&(x, y)) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!("");
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

fn day11(arr: &mut Vec<i64>, part2: bool) -> HashSet<(i32, i32)> {
	let mut i = 0;
    let mut relative_base = 0;

	let mut robot = (0, 0);
	let mut dir = Dir::Up;
	let mut whites = HashSet::new();
	if part2 {
		whites.insert((0, 0));
	}
	let mut painted = HashSet::new();

	let mut paint = true;

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			return if part2 { whites } else { painted };
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
				let input = if whites.contains(&robot) { 1 } else { 0 };
				store(pos, input, arr);
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], arr, relative_base);
				if paint {
					if output == 0 {
						whites.remove(&robot);
					} else {
						whites.insert(robot);
					}
					painted.insert(robot);
				} else {
					if output == 0 {
						// Turn left
						dir = match dir {
								Dir::Up => Dir::Left,
								Dir::Right => Dir::Up,
								Dir::Down => Dir::Right,
								Dir::Left => Dir::Down,
							};
					} else {
						// Turn right
						dir = match dir {
								Dir::Up => Dir::Right,
								Dir::Right => Dir::Down,
								Dir::Down => Dir::Left,
								Dir::Left => Dir::Up,
							};
					}
					
					robot = match dir {
						Dir::Up => (robot.0, robot.1 - 1),
						Dir::Right => (robot.0 + 1, robot.1),
						Dir::Down => (robot.0, robot.1 + 1),
						Dir::Left => (robot.0 - 1, robot.1),
					};
				}
				paint = !paint;
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