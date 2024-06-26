use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, fs::read_to_string};

#[derive(Clone, Eq, PartialEq)]
struct State {
	steps: usize,
	keys: Vec<char>,
	pos: Vec<(usize, usize)>,
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

	let part1 = day18(map.clone());
	println!("Day 18 part 1: {}", part1);

	let start = *map.iter().find(|(_, &c)| c == '@').unwrap().0;

	let mut map = map;

	*map.get_mut(&(start.0, start.1)).unwrap() = '#';
	*map.get_mut(&(start.0 + 1, start.1)).unwrap() = '#';
	*map.get_mut(&(start.0 - 1, start.1)).unwrap() = '#';
	*map.get_mut(&(start.0, start.1 + 1)).unwrap() = '#';
	*map.get_mut(&(start.0, start.1 - 1)).unwrap() = '#';
	
	*map.get_mut(&(start.0 + 1, start.1 + 1)).unwrap() = '@';
	*map.get_mut(&(start.0 + 1, start.1 - 1)).unwrap() = '@';
	*map.get_mut(&(start.0 - 1, start.1 + 1)).unwrap() = '@';
	*map.get_mut(&(start.0 - 1, start.1 - 1)).unwrap() = '@';

	let part2 = day18(map);
	println!("Day 18 part 2: {}", part2);
}

fn day18(map: HashMap<(usize, usize), char>) -> usize {

    let start = map.iter().filter(|(_, &c)| c == '@').map(|(&x, _)| x).collect::<Vec<_>>();
	let all_keys = map.values().filter(|x| x.is_ascii_lowercase()).map(|&x| x).collect::<Vec<_>>();

	let mut queue = BinaryHeap::new();
	let mut visited = (0..start.len()).map(|_| HashMap::new()).collect::<Vec<_>>();

	queue.push(State { steps: 0, keys: Vec::new(), pos: start});

	while let Some(state) = queue.pop() {
		if all_keys.iter().all(|c| state.keys.contains(c)) {
			return state.steps;
		}

		for i in 0..state.pos.len() {

			let k = (ku32(&&all_keys, &state.keys), state.pos[i]);
	
			if visited[i].contains_key(&k) && state.steps > visited[i][&k] { continue; }

			let pos = state.pos[i];
			let neighs = [
				(pos.0, pos.1 + 1),
				(pos.0, pos.1 - 1),
				(pos.0 + 1, pos.1),
				(pos.0 - 1, pos.1),
			];
	
			for neigh in neighs {
				let c = map[&neigh];
				if c == '#' { continue; }
	
				let mut keys = state.keys.clone();
				let steps = state.steps + 1;
				if c.is_ascii_uppercase() && !state.keys.contains(&c.to_ascii_lowercase()) {
					continue;
				}

				let mut new_pos = Vec::new();
				for j in 0..state.pos.len() {
					if i == j {
						new_pos.push(neigh);
					} else {
						new_pos.push(state.pos[j]);
					}
				}
	
				let k = (ku32(&all_keys, &keys), neigh);
				if visited[i].contains_key(&k) && visited[i][&k] <= steps { continue; }
	
				if c.is_ascii_lowercase() {
					keys.push(c);
				}
	
				queue.push(State {
					steps: state.steps + 1,
					keys: keys,
					pos: new_pos,
				});
	
				visited[i].insert(k, steps);
			}
		}
	}

	unreachable!("No end");
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