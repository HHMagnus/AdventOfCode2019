
use std::fs::read_to_string;

#[derive(Debug)]
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

	for line in lines {
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
}