use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day18.txt").unwrap();

    let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(|(x, c)| ((x, y), c))).collect::<HashMap<_,_>>();

    let start = *map.iter().find(|(_, &c)| c == '@').unwrap().0;
}