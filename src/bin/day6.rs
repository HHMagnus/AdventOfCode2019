use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day6.txt").unwrap();

	let mut deps = HashMap::new();

	for x in file.lines() {
		let split = x.split(")").collect::<Vec<_>>();
		deps.entry(split[1]).or_insert(Vec::new()).push(split[0]);
		deps.entry(split[0]).or_insert(Vec::new());
	}

	let mut prevs = HashMap::new();

	let mut part1 = 0;

	while deps.len() > 0 {
		let next = *deps.iter().find(|x| x.1.is_empty()).unwrap().0;
		deps.remove(next);

		let mut others = 0;
		let curr = *prevs.get(&next).unwrap_or(&0);
		part1 += curr;

		while let Some(other) =  deps.iter_mut().find(|x| x.1.contains(&next)) {
			let idx = other.1.iter().position(|&x| x == next).unwrap();
			other.1.remove(idx);
			others += 1;
			*prevs.entry(*other.0).or_default() += curr + 1;
		}

		*prevs.entry(next).or_default() = others;
	}

	println!("Day 6 part 1: {}", part1);
}