
use std::fs::read_to_string;
use mod_exp::mod_exp;

#[derive(Debug, Clone)]
enum CardAction {
	DealIntoNewStack,
	Cut(i32),
	DealWithIncrement(i32)
}

fn main() {
    let file = read_to_string("input/day22.txt").unwrap();

	let lines = file.lines().map(|x| {
		if x.starts_with("deal into new stack") {
			CardAction::DealIntoNewStack
		} else if x.starts_with("cut ") {
			CardAction::Cut(x.replace("cut ", "").parse::<i32>().unwrap())
		} else if x.starts_with("deal with increment ") {
			CardAction::DealWithIncrement(x.replace("deal with increment ", "").parse::<i32>().unwrap())
		} else {
			panic!("Unknown input line: {}", x);
		}
	}).collect::<Vec<_>>();

	let mut cards = (0..10007).into_iter().collect::<Vec<_>>();

	for line in lines.clone() {
		match line {
			CardAction::DealIntoNewStack => {
				cards.reverse();
			},
			CardAction::Cut(x) => {
				if x.is_positive() {
					cards = cards[x as usize..cards.len()].iter().chain(cards[0..x as usize].iter()).map(|x| *x).collect();
				} else {
					let x = x.abs();
					cards = cards[cards.len() - x as usize..cards.len()].iter().chain(cards[0..cards.len() - x as usize].iter()).map(|x| *x).collect();
				}
			},
			CardAction::DealWithIncrement(x) => {
				let mut new = Vec::with_capacity(10007);

				for x in 0..10007 {
					new.push(x);
				}

				let mut i = 0;
				
				for card in cards {
					new[i % 10007] = card;
					i += x as usize;
				}

				cards = new;
			},
		}
	}

	let part1 = cards.into_iter().enumerate().find(|(_, x)| x == &2019).unwrap().0;
	println!("Day 22 part 1: {}", part1);

	let deck_size = 119315717514047i128;
	let times = 101741582076661i128;

	let mut offset_diff = 0;
	let mut increment_mul = 1;

	for line in lines.clone() {
		match line {
			CardAction::DealIntoNewStack => {
				increment_mul = -increment_mul;
				offset_diff += increment_mul;
			},
			CardAction::Cut(x) => {
				offset_diff += increment_mul * x as i128;
			},
			CardAction::DealWithIncrement(x) => {
				increment_mul *= mod_exp(x as i128, deck_size-2, deck_size);
			},
		}
		offset_diff %= deck_size;
		increment_mul %= deck_size;
	}

	let increment = 2020 * mod_exp(increment_mul, times, deck_size) % deck_size;
	let offset = offset_diff * ((1 - mod_exp(increment_mul, times, deck_size)) * mod_exp(1 - increment_mul, deck_size-2, deck_size) % deck_size);
	let part2 = (offset + increment) % deck_size;
	println!("Day 22 part 2: {}", part2);
	// Explanation on reddit: https://www.reddit.com/r/adventofcode/comments/ee0rqi/comment/fbnkaju/
}