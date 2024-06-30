use std::{collections::{BTreeSet, HashMap, VecDeque}, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day20.txt").unwrap();

	let char_map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();
	
	let open_spaces = char_map.iter().filter(|(_, &x)| x == '.').map(|(x, _)| *x).collect::<Vec<_>>();

	let letters = char_map.iter().filter(|(_, c)| c.is_ascii_uppercase()).collect::<Vec<_>>();

	let mut portal_to = Vec::new();
	let mut portal_from = HashMap::new();

	for &(&(x, y), &c) in &letters {
		let neighs = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
		let (&(x0, y0), &c0) = *letters.iter().find(|(xy, _)| neighs.contains(&xy)).unwrap();
		let mut positioning = [((x, y), c), ((x0, y0), c0)].to_vec();
		positioning.sort_by_key(|(xy, _)| *xy);
		let neighs0 = [(x0 - 1, y0), (x0 + 1, y0), (x0, y0 - 1), (x0, y0 + 1)];
		let other_neighs = neighs0.iter().chain(neighs.iter()).cloned().collect::<Vec<_>>();
		let &open = open_spaces.iter().find(|xy| other_neighs.contains(xy)).unwrap();
		let mut str = String::new();
		str.push(positioning[0].1);
		str.push(positioning[1].1);
		let from = if neighs.contains(&open) { (x, y) } else { (x0, y0) };
		portal_from.insert(from, str.clone());
		portal_to.push((str, open));
	}

	// Part 1
	let start = portal_to.iter().find(|x| x.0 == "AA").unwrap().1;
	let mut queue = VecDeque::new();
	let mut visited = BTreeSet::new();
	visited.insert(start);
	queue.push_back((start, 0));

	while let Some((next, steps)) = queue.pop_front() {
		for neighbor in [(next.0 - 1, next.1), (next.0 + 1, next.1), (next.0, next.1 - 1), (next.0, next.1 + 1)] {
			if visited.contains(&neighbor) {
				continue;
			}
			visited.insert(neighbor);

			if open_spaces.contains(&neighbor) {
				queue.push_back((neighbor, steps+1));
				continue;
			}

			if portal_from.contains_key(&neighbor) {
				let portal = portal_from.get(&neighbor).unwrap();
				if portal == "ZZ" {
					println!("Day 20 part 1: {}", steps);
					break;
				}
				if portal == "AA" {
					continue;
				}

				let other_side = portal_to.iter().find(|(str, xy)| str == portal && xy != &next).unwrap();
				queue.push_back((other_side.1, steps + 1));
			}
		}
	}

	// Part 2
	let &maxx = open_spaces.iter().map(|(x, _)| x).max().unwrap();
	let &maxy = open_spaces.iter().map(|(_, y)| y).max().unwrap();
	let mut queue = VecDeque::new();
	let mut visited = BTreeSet::new();
	let start = (start.0, start.1, 0);
	queue.push_back((start, 0));

	'p2: while let Some((next, steps)) = queue.pop_front() {
		if visited.contains(&next) {
			continue;
		}
		visited.insert(next);
		for neighbor in [(next.0 - 1, next.1), (next.0 + 1, next.1), (next.0, next.1 - 1), (next.0, next.1 + 1)] {
			if open_spaces.contains(&neighbor) {
				queue.push_back(((neighbor.0, neighbor.1, next.2), steps + 1));
				continue;
			}

			if portal_from.contains_key(&neighbor) {
				let portal = portal_from.get(&neighbor).unwrap();
				if portal == "AA" {
					continue;
				}

				if portal == "ZZ" {
					if next.2 != 0 {
						continue;
					}
					println!("Day 20 part 2: {}", steps);
					break 'p2;
				}

				let other_side = portal_to.iter().find(|(str, xy)| str == portal && xy != &(next.0, next.1)).unwrap();
				let outside_portal = !(other_side.1.0 <= 2 || other_side.1.0 >= maxx || other_side.1.1 <= 2 || other_side.1.1 >= maxy);

				if outside_portal && next.2 == 0 {
					continue;
				}

				let level = next.2 + if outside_portal { -1 } else { 1 };

				queue.push_back(((other_side.1.0, other_side.1.1, level), steps + 1));
			}
		}
	}
}