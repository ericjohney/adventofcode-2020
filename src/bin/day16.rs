use adventofcode2020::utils::file;
use parse_display::{Display, FromStr};
use std::cmp;
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{name}: {start1}-{end1} or {start2}-{end2}")]
struct Rule {
	name: String,
	start1: u64,
	end1: u64,
	start2: u64,
	end2: u64,
}

impl Rule {
	fn is_valid(&self, num: &u64) -> bool {
		let range1 = self.start1..=self.end1;
		let range2 = self.start2..=self.end2;
		range1.contains(num) || range2.contains(num)
	}
}

fn main() {
	let input = file::read_to_string("inputs/day16.txt")
		.trim()
		.split("\n\n")
		.map(|s| s.to_string())
		.collect::<Vec<_>>();

	part1(&input);
	part2(&input);
}

fn part1(input: &Vec<String>) {
	let rules: Vec<Rule> = input[0]
		.split("\n")
		.map(|l| l.parse().ok().unwrap())
		.collect();

	let nearby_tickets: Vec<Vec<u64>> = input[2]
		.split("\n")
		.skip(1)
		.map(|l| {
			l.split(",")
				.map(|str_num| str_num.parse().ok().unwrap())
				.collect()
		})
		.collect();

	let error_rate: u64 = nearby_tickets
		.iter()
		.flatten()
		.filter(|&num| !rules.iter().any(|rule| rule.is_valid(num)))
		.sum();
	println!("part 1 {:?}", error_rate);
}

fn part2(input: &Vec<String>) {
	let rules: Vec<Rule> = input[0]
		.split("\n")
		.map(|l| l.parse().ok().unwrap())
		.collect();

	let nearby_tickets: Vec<Vec<u64>> = input[2]
		.split("\n")
		.skip(1)
		.map(|l| {
			l.split(",")
				.map(|str_num| str_num.parse().ok().unwrap())
				.collect()
		})
		.collect();

	let valid_nearby_tickets = nearby_tickets
		.iter()
		.filter(|ticket| {
			!ticket
				.iter()
				.any(|num| !rules.iter().any(|rule| rule.is_valid(num)))
		})
		.collect::<Vec<_>>();

	let valid_nearby_ticket_columns = (0..rules.len())
		.map(|i| {
			valid_nearby_tickets
				.iter()
				.map(|ticket| ticket[i])
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let mut available_rules = rules.clone();
	let mut rules_by_column = HashMap::new();
	while available_rules.len() > 0 {
		for (i, column) in valid_nearby_ticket_columns.iter().enumerate() {
			if rules_by_column.contains_key(&i) {
				continue;
			}
			let matches = available_rules
				.iter()
				.cloned()
				.filter(|rule| column.iter().all(|num| rule.is_valid(num)))
				.collect::<Vec<_>>();

			if matches.len() == 1 {
				let rule = matches[0].clone();
				let rule_name = rule.name.to_string();
				rules_by_column.insert(i, rule);
				available_rules = available_rules
					.clone()
					.iter()
					.cloned()
					.filter(|a| a.name != rule_name)
					.collect();
			}
		}
	}

	let departure_columns = rules_by_column
		.iter()
		.filter(|(_, rule)| rule.name.starts_with("departure"))
		.map(|(&column, _)| column)
		.collect::<Vec<_>>();

	let my_ticket: Vec<u64> = input[1]
		.split("\n")
		.skip(1)
		.last()
		.unwrap()
		.split(",")
		.map(|str_num| str_num.parse().ok().unwrap())
		.collect();
	let product: u64 = departure_columns.iter().map(|&c| my_ticket[c]).product();

	println!("part 2 {:?}", product);
}
