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
	let mut part2s: HashMap<&str, Vec<&str>> = HashMap::new();

	let mut part1 = 0;

	while deps.len() > 0 {
		let next = *deps.iter().find(|x| x.1.is_empty()).unwrap().0;
		deps.remove(next);

		let mut others = 0;
		let curr = *prevs.get(&next).unwrap_or(&0);
		part1 += curr;
		let prev = part2s.get(&next).map(|x| x.clone()).unwrap_or(Vec::new());

		while let Some(other) =  deps.iter_mut().find(|x| x.1.contains(&next)) {
			let idx = other.1.iter().position(|&x| x == next).unwrap();
			other.1.remove(idx);
			others += 1;
			*prevs.entry(*other.0).or_default() += curr + 1;
			part2s.entry(*other.0).or_insert(prev.clone()).push(next);
		}

		*prevs.entry(next).or_default() = others;
	}

	let mut back_you = part2s["YOU"].clone();
	back_you.reverse();
	let mut back_san = part2s["SAN"].clone();
	back_san.reverse();

	let by = back_you.iter().enumerate().filter(|(_, x)| back_san.contains(&x)).min_by_key(|x| x.0).unwrap().0;
	let bs = back_san.iter().enumerate().filter(|(_, x)| back_you.contains(&x)).min_by_key(|x| x.0).unwrap().0;

	let part2 = by + bs;

	println!("Day 6 part 1: {}", part1);
	println!("Day 6 part 1: {}", part2);
}