use adventofcode2020::utils::file;
use std::cmp;
use std::collections::HashMap;

fn main() {
	let input = file::read_to_string("inputs/day15.txt")
		.trim()
		.split(",")
		.map(|num| str::parse::<i64>(num).unwrap())
		.collect::<Vec<_>>();

	part1(&input);
	part2(&input);
}

fn part1(input: &Vec<i64>) {
	let mut spoken = input.clone();

	for turn in input.len()..2020 {
		let last_number = *spoken.last().unwrap();
		let last_number_last_spoken = spoken
			.iter()
			.enumerate()
			.rev()
			.skip(1)
			.find(|(_, &x)| x == last_number);
		if last_number_last_spoken.is_some() {
			let last_turn = last_number_last_spoken.unwrap().0 as i64;
			spoken.push((turn as i64) - 1 - last_turn);
		} else {
			spoken.push(0);
		}
	}
	println!("part 1 {:?}", spoken.last().unwrap());
}

fn part2(input: &Vec<i64>) {
	let max_turns = 30000000;
	let mut spoken = input
		.iter()
		.enumerate()
		.map(|(i, &num)| (num, i + 1))
		.collect::<HashMap<_, _>>();

	let mut last_number = *input.last().unwrap();
	for turn in (input.len() + 1)..=max_turns {
		let last_number_last_spoken = spoken.get(&last_number);
		let next_number;
		if last_number_last_spoken.is_some() && *last_number_last_spoken.unwrap() < turn - 1 {
			let last_turn = *last_number_last_spoken.unwrap();
			next_number = (turn as i64) - 1 - (last_turn as i64);
		} else {
			next_number = 0;
		}
		spoken.insert(last_number, turn - 1);
		last_number = next_number;
	}
	println!("part 2 {:?}", last_number);
}
