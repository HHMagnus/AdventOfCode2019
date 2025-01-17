use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day16.txt").unwrap();

    let input = [0, 1, 0, -1];

    let list = file.chars().map(|x| x.to_digit(10).unwrap() as i64).collect::<Vec<_>>();

    let mut part1 = list.clone();

    for _ in 0..100 {
        part1 = (0..part1.len()).into_iter().map(|p| {
            let value = part1.iter().enumerate().map(|(i, x)| input[((i+1) / (p+1)) % input.len()] * x).sum::<i64>();
            value.abs() % 10
        }).collect();
    }

    let part1 = part1[..8].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");

    println!("Day 16 part 1: {}", part1);

    let part2 = list.clone().repeat(10000);
    let index = part2[..7].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("").parse::<usize>().unwrap();
    let mut part2 = part2[index..].to_vec();
    
    for _ in 0..100 {
        let mut total = 0;
        for i  in (0..part2.len()).rev() {
            total += part2[i];
            part2[i] = total % 10;
        }
    }

    let part2 = part2[..8].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");

    println!("Day 16 part 2: {}", part2);
}