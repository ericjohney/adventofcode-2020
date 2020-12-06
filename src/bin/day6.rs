use adventofcode2019::file_utils;
use std::collections::HashSet;

fn main() {
	let groups = file_utils::read_to_string("inputs/day6.txt")
		.split("\n\n")
		.map(|group| group.to_string())
		.collect();

	part1(&groups);
	part2(&groups);
}

fn part1(groups: &Vec<String>) {
	let mut yes_count = 0;
	for group in groups {
		yes_count += group
			.replace("\n", "")
			.chars()
			.collect::<HashSet<_>>()
			.len();
	}
	println!("part1 yes count {}", yes_count);
}

fn part2(groups: &Vec<String>) {
	let mut yes_count = 0;
	for group in groups {
		let people = group
			.split("\n")
			.map(|person| person.chars().collect::<HashSet<_>>())
			.collect::<Vec<_>>();

		let mut shared = people[0].clone();
		for person in people {
			shared = shared.intersection(&person).cloned().collect();
		}
		yes_count += shared.len();
	}
	println!("part2 yes count {}", yes_count);
}
