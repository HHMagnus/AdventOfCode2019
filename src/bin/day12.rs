use std::{collections::HashMap, fs::read_to_string};

fn change (x1: i64, x2: i64) -> i64 {
	if x1 < x2 { 1 } else if x1 > x2 { -1 } else { 0 }
}
fn main() {
    let file = read_to_string("input/day12.txt").unwrap();

	let mut map1 = HashMap::new();
	let mut map2 = HashMap::new();
	let mut map3 = HashMap::new();

	let mut moons = file.lines().map(|x| {
		let removed = x.replace("<x=", "").replace(" y=", "").replace(" z=", "").replace(">", "");
		let split = removed.split(",").map(|y| y.parse::<i64>().unwrap()).collect::<Vec<_>>();
		((split[0], split[1], split[2]), (0, 0, 0))
	}).collect::<Vec<_>>();

	let mut x1 = 0;
	let mut y1 = 0;
	let mut z1 = 0;

	let mut i = 0;

	loop {
		let x = moons.iter().map(|x| (x.0.0, x.1.0)).collect::<Vec<_>>();
		if map1.contains_key(&x) {
			x1 = i - map1[&x];
		}
		map1.insert(x, i);

		let y = moons.iter().map(|x| (x.0.1, x.1.1)).collect::<Vec<_>>();
		if map2.contains_key(&y) {
			y1 = i - map2[&y];
		}
		map2.insert(y, i);
		let z = moons.iter().map(|x| (x.0.2, x.1.2)).collect::<Vec<_>>();
		if map3.contains_key(&z) {
			z1 = i - map3[&z];
		}
		map3.insert(z, i);

		if x1 != 0 && y1 != 0 && z1 != 0 {
			println!("Day 12 part 2: lcm({}, {}, {})", x1, y1, z1);
			break;
		}

		let positions = moons.iter().map(|x| x.0).collect::<Vec<_>>();
		for moon in &mut moons {
			for pos in &positions {
				moon.1 = (moon.1.0 + change(moon.0.0, pos.0), moon.1.1 + change(moon.0.1, pos.1), moon.1.2 + change(moon.0.2, pos.2));
			}

			moon.0 = (moon.0.0 + moon.1.0, moon.0.1 + moon.1.1, moon.0.2 + moon.1.2)
		}
		i += 1;

		if i == 1000 {
			let part1 = moons.iter().map(|&((x, y, z), (vx, vy, vz))| (x.abs() + y.abs() + z.abs()) * (vx.abs() + vy.abs() + vz.abs())).sum::<i64>();
			println!("Day 12 part 1: {}", part1);
		}
	}

	
}