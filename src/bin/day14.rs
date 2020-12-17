use adventofcode2020::utils::file;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("mask = {raw}")]
struct Mask {
	raw: String,
}
#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("mem[{addr}] = {val}")]
struct Command {
	addr: usize,
	val: usize,
}

fn main() {
	let input = file::lines("inputs/day14.txt");

	part1(&input);
	part2(&input);
}

fn part1(input: &Vec<String>) {
	fn parse_mask(input: String) -> Vec<(u64, u64)> {
		let mask: Mask = input.parse().ok().unwrap();
		mask
			.raw
			.chars()
			.rev()
			.enumerate()
			.filter(|(_, c)| *c != 'X')
			.map(|(i, c)| (i as u64, c.to_digit(2 as u32).unwrap() as u64))
			.collect::<Vec<_>>()
	}

	let mut mem_space: HashMap<u64, u64> = HashMap::new();
	let mut current_mask = Vec::new();

	for line in input {
		if line.starts_with("mask") {
			current_mask = parse_mask(line.to_string());
			continue;
		}

		let command: Command = line.parse().ok().unwrap();

		let val = format!("{:0>36b}", command.val)
			.chars()
			.rev()
			.enumerate()
			.map(|(i, c)| {
				let mut val = c.to_digit(2u32).unwrap() as u64;
				for (index, new_val) in &current_mask {
					if *index == i as u64 {
						val = *new_val;
						break;
					}
				}
				return val * 2u64.pow(i as u32);
			})
			.sum();

		mem_space.insert(command.addr as u64, val);
	}

	println!("part1 {:?}", mem_space.values().sum::<u64>());
}

fn part2(input: &Vec<String>) {
	fn findAddressVariants(expandedAddress: String) -> Vec<String> {
		// Find first occurrence of a floating bit.
		let i = expandedAddress.find('X');

		// No floating bit found, we can return the address.
		if i.is_none() {
			return vec![expandedAddress];
		}

		// Find all addresses where this floating bit is `"0"`.
		let with0 = findAddressVariants(
			expandedAddress
				.chars()
				.enumerate()
				.map(|(idx, c)| if idx == i.unwrap() { '0' } else { c })
				.collect(),
		);

		// Find all addresses where this floating bit is `"1"`.
		let with1 = findAddressVariants(
			expandedAddress
				.chars()
				.enumerate()
				.map(|(idx, c)| if idx == i.unwrap() { '1' } else { c })
				.collect(),
		);

		// Merge results.
		[with0, with1].concat()
	}

	let mut mem_space: HashMap<u64, u64> = HashMap::new();
	let mut current_mask = Mask {
		raw: "".to_string(),
	};

	for line in input {
		if line.starts_with("mask") {
			current_mask = line.parse().ok().unwrap();
			continue;
		}

		let command: Command = line.parse().ok().unwrap();

		let address: String = format!("{:0>36b}", command.addr)
			.chars()
			.enumerate()
			.map(|(i, c)| match current_mask.raw.chars().nth(i).unwrap() {
				'X' => 'X',
				'1' => '1',
				_ => c,
			})
			.collect();

		for addr in findAddressVariants(address) {
			let int_addr = u64::from_str_radix(addr.as_str(), 2).unwrap();
			mem_space.insert(int_addr, command.val as u64);
		}
	}

	println!("part1 {:?}", mem_space.values().sum::<u64>());
}
