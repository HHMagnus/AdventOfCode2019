use std::fs::read_to_string;

fn change (x1: i32, x2: i32) -> i32 {
	if x1 < x2 { 1 } else if x1 > x2 { -1 } else { 0 }
}
fn main() {
    let file = read_to_string("input/day12.txt").unwrap();

	let mut moons = file.lines().map(|x| {
		let removed = x.replace("<x=", "").replace(" y=", "").replace(" z=", "").replace(">", "");
		let split = removed.split(",").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>();
		((split[0], split[1], split[2]), (0, 0, 0))
	}).collect::<Vec<_>>();

	for _ in 0..1000 {
		let positions = moons.iter().map(|x| x.0).collect::<Vec<_>>();
		for moon in &mut moons {
			for pos in &positions {
				if *pos == moon.0 { continue; }
				moon.1 = (moon.1.0 + change(moon.0.0, pos.0), moon.1.1 + change(moon.0.1, pos.1), moon.1.2 + change(moon.0.2, pos.2));
			}

			moon.0 = (moon.0.0 + moon.1.0, moon.0.1 + moon.1.1, moon.0.2 + moon.1.2)
		}
	}

	let part1 = moons.iter().map(|&((x, y, z), (vx, vy, vz))| (x.abs() + y.abs() + z.abs()) * (vx.abs() + vy.abs() + vz.abs())).sum::<i32>();
	println!("Day 12 part 1: {}", part1);
}