use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day4.txt").unwrap();
    let split = file.split("-").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;
    
    for x in split[0]..=split[1] {
        let chars = x.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>();
        let two_equal = chars.as_slice().windows(2).any(|x| x[0] == x[1]);
        let less_or_equal = chars.as_slice().windows(2).all(|x| x[0] <= x[1]);
        if two_equal && less_or_equal {
            part1 += 1;
        }

        let not_part_of_group = chars.as_slice().windows(4).any(|x| x[0] != x[1] && x[1] == x[2] && x[2] != x[3]);
        let start = chars[0] == chars[1] && chars[1] != chars[2];
        let end = chars[chars.len()-1] == chars[chars.len()-2] && chars[chars.len()-2] != chars[chars.len()-3];

        if two_equal && less_or_equal && (not_part_of_group || start || end) {
            part2 += 1;
        }
    }

    println!("Day 4 part 1: {}", part1);
    println!("Day 4 part 2: {}", part2);
}