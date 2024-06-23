use std::{collections::HashMap, fs::read_to_string};

pub fn pop<T, B>(map: &mut HashMap<T, B>) -> Option<(T, B)>
where
	T: Eq + Clone + std::hash::Hash,
	B: Eq + Clone + std::hash::Hash,
{
    let elt = map.iter().next()?;
	let clone = (elt.0.clone(), elt.1.clone());
    map.remove(&clone.0);
	Some(clone)
}

fn main() {
    let file = read_to_string("input/day14.txt").unwrap().leak();

	let input = file.lines().map(|x| {
		let split = x.split(" => ").collect::<Vec<_>>();
		let first = split[0].split(", ").collect::<Vec<_>>();
		let second = split[1].split(" ").collect::<Vec<_>>();
		let first = first.into_iter().map(|f| {
			let s = f.split(" ").collect::<Vec<_>>();
			(s[0].parse::<usize>().unwrap(), s[1])
		}).collect::<Vec<_>>();
		(first, ((second[0].parse::<usize>().unwrap(), second[1])))
	}).collect::<Vec<_>>();

	let part1 = day14(input.clone(), 1);
	println!("Day 14 part 1: {}", part1);

	let mut min = 0;
	let mut max = 1000000000000 as u128;

	while min != max {
		let mid = (max + min) / 2;
		
		let ore = day14(input.clone(), mid);

		if ore > 1000000000000 {
			max = mid - 1;
		} else if ore < 1000000000000 {
			min = mid + 1;
		} else {
			max = mid;
			min = mid;
		}
	}

	println!("Day 14 part 2: {}", min);
}

fn day14(input: Vec<(Vec<(usize, &str)>, (usize, &str))>, fuel: u128) -> u128 {
	let mut dep_map = HashMap::new();

	for i in &input {
		*dep_map.entry(i.1.1).or_insert(Vec::new()) = i.0.iter().map(|x| x.1).collect::<Vec<_>>();
	}

	let mut ore: u128 = 0;
	let mut collection = HashMap::new();
	collection.insert("FUEL", fuel);

	while let Some((&dep, _)) = dep_map.iter().find(|&x| !dep_map.iter().any(|y| y.1.contains(x.0)) && collection.contains_key(x.0)) {
		dep_map = dep_map.into_iter().map(|x| (x.0, x.1.into_iter().filter(|&x| x != dep).collect())).collect();
		dep_map.remove(dep);

		if let Some(&next) = collection.get(dep) {
			let i = input.iter().find(|x| x.1.1 == dep).unwrap();
	
			let diff = next / (i.1.0 as u128) + if next % (i.1.0 as u128) > 0 { 1 } else { 0 };
	
			for x in i.0.iter() {
				if x.1 == "ORE" {
					ore += x.0 as u128 * diff as u128;
					continue;
				}
	
				*collection.entry(x.1).or_insert(0) += x.0 as u128 * diff as u128;
			}
			
		}
	}

	ore
}