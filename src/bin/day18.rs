use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, fs::read_to_string};

#[derive(Clone, Eq, PartialEq)]
struct State {
	steps: usize,
	keys: Vec<char>,
	pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file = read_to_string("input/day18.txt").unwrap();

    let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x, y), c))).collect::<HashMap<_,_>>();

    let start = *map.iter().find(|(_, &c)| c == '@').unwrap().0;
	let all_keys = map.values().filter(|x| x.is_ascii_lowercase()).map(|&x| x).collect::<Vec<_>>();

	let mut queue = BinaryHeap::new();
	let mut visited = HashMap::new();

	queue.push(State { steps: 0, keys: Vec::new(), pos: start});

	while let Some(state) = queue.pop() {
		if all_keys.iter().all(|c| state.keys.contains(c)) {
			println!("Day 18 part 1: {}", state.steps);
			break;
		}

		let k = (ku32(&&all_keys, &state.keys), state.pos.clone());

		if visited.contains_key(&k) && state.steps > visited[&k] { continue; }

		let neighs = [
			(state.pos.0, state.pos.1 + 1),
			(state.pos.0, state.pos.1 - 1),
			(state.pos.0 + 1, state.pos.1),
			(state.pos.0 - 1, state.pos.1),
		];

		for neigh in neighs {
			let c = map[&neigh];
			if c == '#' { continue; }

			let mut keys = state.keys.clone();
			let steps = state.steps + 1;
			if c.is_ascii_uppercase() && !state.keys.contains(&c.to_ascii_lowercase()) {
				continue;
			}

			let k = (ku32(&all_keys, &keys), neigh);
			if visited.contains_key(&k) && visited[&k] <= steps { continue; }

			if c.is_ascii_lowercase() {
				keys.push(c);
			}

			queue.push(State {
				steps: state.steps + 1,
				keys: keys,
				pos: neigh,
			});

			visited.insert(k, steps);
		}
	}
}

fn ku32(all: &Vec<char>, keys: &Vec<char>) -> u32 {
	let mut res = 0;
	for x in all {
		if keys.contains(x) {
			res |= 1;
		}
		res <<= 1;
	}
	res
}