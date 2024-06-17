use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
enum Direction {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    let file = read_to_string("input/day3.txt").unwrap();

    let lines = file.lines().map(|line| {
        line.split(",").map(|dir| {
            let num = dir[1..].parse::<i32>().unwrap();
            if dir.starts_with("R") {
                Direction::Right(num)
            } else if dir.starts_with("L") {
                Direction::Left(num)
            } else if dir.starts_with("U") {
                Direction::Up(num)
            } else if dir.starts_with("D") {
                Direction::Down(num)
            } else {
                unreachable!("Unknown direction {}", dir)
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    
    let mut reached = HashMap::new();
    let mut curr = (0, 0);
    let mut count = 0;

    for x in &lines[0] {
        let &c = match x {
            Direction::Right(x) => x,
            Direction::Left(x) => x,
            Direction::Up(x) => x,
            Direction::Down(x) => x,
        };

        for _ in 0..c {
            curr = match x {
                Direction::Right(_) => (curr.0 + 1, curr.1),
                Direction::Left(_) => (curr.0 - 1, curr.1),
                Direction::Up(_) => (curr.0, curr.1 - 1),
                Direction::Down(_) => (curr.0, curr.1 + 1),
            };
            count += 1;
            reached.insert(curr, count);
        }
    }

    let mut part1 = i32::MAX;
    let mut part2 = i32::MAX;
    let mut curr = (0, 0);
    let mut count = 0;

    for x in &lines[1] {
        let &c = match x {
            Direction::Right(x) => x,
            Direction::Left(x) => x,
            Direction::Up(x) => x,
            Direction::Down(x) => x,
        };

        for _ in 0..c {
            curr = match x {
                Direction::Right(_) => (curr.0 + 1, curr.1),
                Direction::Left(_) => (curr.0 - 1, curr.1),
                Direction::Up(_) => (curr.0, curr.1 - 1),
                Direction::Down(_) => (curr.0, curr.1 + 1),
            };
            count += 1;
            if reached.contains_key(&curr) {
                part1 = part1.min(man_dist((0, 0), curr));
                let other = reached[&curr];
                part2 = part2.min(other + count);
            }
        }
    }

    println!("Day 3 part 1: {}", part1);
    println!("Day 3 part 2: {}", part2);
}

fn man_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}