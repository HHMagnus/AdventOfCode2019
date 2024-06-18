use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day5.txt").unwrap();

    let mut vec = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

	day5(&mut vec);
}

fn load(param: i32, arg: i32, arr: &mut Vec<i32>) -> i32 {
	if param == 0 {
		arr[arg as usize]
	} else {
		arg
	}
}

fn day5(arr: &mut Vec<i32>) {
	let mut i = 0;

	loop {
		let op_code = arr[i] % 100;
        let param1 = arr[i] / 100 % 1000;
        let param2 = arr[i] / 1000 % 10000;
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
				let input = 1;
				let pos = arr[i+1] as usize;
				arr[pos] = input;
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], arr);
				println!("{}", output);
				i += 2;
			},
			x => unreachable!("Unknown opcode {}", x),
		}
	}
}