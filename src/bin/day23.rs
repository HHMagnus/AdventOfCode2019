
use std::{collections::{HashMap, VecDeque}, fs::read_to_string};

struct Packet {
	computer_id: i64,
	x: i64,
	y: i64,
}

struct Computer {
	code: Vec<i64>,
	relative_base: i64,
	instruction_pointer: usize,
}

struct Network {
	packets: VecDeque<Packet>,
	computers: HashMap<i64, Computer>,
}

impl Packet {
	fn construct(output: Vec<i64>) -> VecDeque<Packet> {
		output.as_slice().chunks(3).map(|x| Packet { computer_id: x[0], x: x[1], y: x[2]}).collect()
	}
}

impl Computer {
	fn new(computer_id: i64, code: Vec<i64>) -> (Computer, VecDeque<Packet>) {
		let (code, ip, rb, out) = day23(code, 0, 0, vec![computer_id, -1]);
		let computer = Computer {
			code,
			relative_base: rb,
			instruction_pointer: ip,
		};
		(computer, Packet::construct(out))
	}

	fn receive(&mut self, packet: Packet) -> VecDeque<Packet> {
		let (code, ip, rb, out) = day23(self.code.clone(), self.instruction_pointer, self.relative_base, vec![packet.x, packet.y]);
		self.code = code;
		self.instruction_pointer = ip;
		self.relative_base = rb;
		Packet::construct(out)
	}
}

impl Network {
	fn new(code: Vec<i64>) -> Network {
		let mut packets = VecDeque::new();
		let mut computers = HashMap::new();
		for ci in 0..50 {
			let (computer, mut packs) = Computer::new(ci, code.clone());
			packets.append(&mut packs);
			computers.insert(ci, computer);
		}
		Network {
			packets,
			computers
		}
	}

	fn run(&mut self) -> i64 {
		while let Some(packet) = self.packets.pop_front() {
			if packet.computer_id == 255 {
				return packet.y;
			}

			let computer = self.computers.get_mut(&packet.computer_id).unwrap();
			let mut packs = computer.receive(packet);
			self.packets.append(&mut packs);
		}

		unreachable!("No Y=255");
	}
}

fn main() {
    let file = read_to_string("input/day23.txt").unwrap();
    let orr_vec = file.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

	let mut network = Network::new(orr_vec);
	let part1 = network.run();
	println!("Day 23 part 1: {:?}", part1);
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

fn day23(mut arr: Vec<i64>, mut i: usize, mut relative_base: i64, input: Vec<i64>) -> (Vec<i64>, usize, i64, Vec<i64>) {
	let mut input_i = 0;

	let mut outgoing = Vec::new();

	loop {
		let op_code = arr[i] % 100;
        let param1 = (arr[i] / 100) % 10;
        let param2 = (arr[i] / 1000) % 10;
        let param3 = arr[i] / 10000 % 10;
		if op_code == 99 {
			return (arr, i, relative_base, outgoing);
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
				if input_i >= input.len() {
					return (arr, i, relative_base, outgoing);
				}
				let pos = pos(param1, arr[i+1], relative_base) as usize;
				let to_store = input[input_i];
				store(pos, to_store, &mut arr);
				input_i += 1;
				i += 2;
			},
			4 => {
				let output = load(param1, arr[i+1], &mut arr, relative_base);
				outgoing.push(output);
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