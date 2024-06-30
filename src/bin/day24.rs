
use std::{collections::BTreeSet, fs::read_to_string};

//A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
//An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.

fn main() {
    part1();
	part2();
}

fn part2() {
	let file = read_to_string("input/day24.txt").unwrap();
	let mut vec = file.lines().into_iter().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter_map(move |(x, c)| if c == '#' { Some((0, x as i32, y as i32)) } else { None })).collect::<BTreeSet<_>>();

	for _ in 0..200 {
		let mut next = BTreeSet::new();
		let depthmin = vec.iter().min_by_key(|x| x.0).unwrap().0;
		let depthmax = vec.iter().max_by_key(|x| x.0).unwrap().0;
		for depth in depthmin-1..=depthmax+2 {
			for y in 0..5 {
				for x in 0..5 {
					if x == 2 && y == 2 { continue; }
					let mut bugs = 0;

					if x == 0 {
						let tile_in_upper = (depth+1, 1, 2);
						if vec.contains(&tile_in_upper) { bugs += 1; }
					} else if x == 4 {
						let tile_in_upper = (depth+1, 3, 2);
						if vec.contains(&tile_in_upper) { bugs += 1; }
					}
					if y == 0 {
						let tile_in_upper = (depth+1, 2, 1);
						if vec.contains(&tile_in_upper) { bugs += 1; }
					} else if y == 4 {
						let tile_in_upper = (depth+1, 2, 3);
						if vec.contains(&tile_in_upper) { bugs += 1; }
					}

					if x == 1 && y == 2 {
						bugs += (0..5).into_iter().map(|y| (depth-1, 0, y)).filter(|pos| vec.contains(pos)).count();
					} else if x == 3 && y == 2 {
						bugs += (0..5).into_iter().map(|y| (depth-1, 4, y)).filter(|pos| vec.contains(pos)).count();
					} else if x == 2 && y == 1 {
						bugs += (0..5).into_iter().map(|x| (depth-1, x, 0)).filter(|pos| vec.contains(pos)).count();
					} else if x == 2 && y == 3 {
						bugs += (0..5).into_iter().map(|x| (depth-1, x, 4)).filter(|pos| vec.contains(pos)).count();
					}

					bugs += [(depth, x - 1, y), (depth, x + 1, y), (depth, x, y - 1), (depth, x, y + 1)].map(|xy| vec.contains(&xy)).iter().filter(|&&x| x).count();

					let curr = (depth, x, y);
					if vec.contains(&curr) {
						if bugs == 1 {
							next.insert(curr);
						}
					} else {
						if bugs == 1 || bugs == 2 {
							next.insert(curr);
						}
					}
				}
			}
		}
		vec = next;
	}
	
	println!("Day 24 part 2: {}", vec.len());
}

fn part1() {
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