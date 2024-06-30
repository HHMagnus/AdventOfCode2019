
use std::{collections::BTreeSet, fs::read_to_string};

//A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
//An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.

fn main() {
    let file = read_to_string("input/day24.txt").unwrap();
	let mut vec = file.lines().into_iter().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter_map(move |(x, c)| if c == '#' { Some((x as i32, y as i32)) } else { None })).collect::<BTreeSet<_>>();

	let mut states = BTreeSet::new();
	loop {
		let mut next = BTreeSet::new();
		for y in 0..5 {
			for x in 0..5 {
				let bugs = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].map(|xy| vec.contains(&xy)).iter().filter(|&&x| x).count();
				if vec.contains(&(x,y)) {
					if bugs == 1 {
						next.insert((x,y));
					}
				} else {
					if bugs == 1 || bugs == 2 {
						next.insert((x,y));
					}
				}
			}
		}
		states.insert(vec);
		vec = next;

		if states.contains(&vec) {
			break;
		}
	}

	let mut part1 = 0;
	let mut field = 1;
	for y in 0..5 {
		for x in 0..5 {
			if vec.contains(&(x, y)) {
				part1 += field;
			}
			field *= 2;
		}
	}

	println!("Day 24 part 1: {}", part1);
}