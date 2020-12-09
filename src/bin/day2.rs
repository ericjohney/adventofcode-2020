extern crate regex;

use adventofcode2020::utils::file;
use std::collections::HashMap;
use std::ops::RangeInclusive;

struct Line {
    start: i64,
    end: i64,
    character: char,
    password: String,
}

fn main() {
    let lines = file::parse_lines("inputs/day2.txt", r"(?m)(\d+)-(\d+) (\w): (\w+)")
        .iter()
        .map(|c| Line {
            start: str::parse::<i64>(&c[1]).unwrap(),
            end: str::parse::<i64>(&c[2]).unwrap(),
            character: c[3].to_string().chars().nth(0).unwrap(),
            password: c[4].to_string(),
        })
        .collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Line>) {
    let valid_count = lines
        .iter()
        .filter(|line| {
            let mut frequency = HashMap::new();
            line.password.chars().for_each(|c| {
                *frequency.entry(c).or_insert(0) += 1;
            });
            let appearances = RangeInclusive::new(line.start, line.end);
            appearances.contains(frequency.get(&line.character).unwrap_or(&0i64))
        })
        .count();
    println!("part1 valid count: {}", valid_count);
}

fn part2(lines: &Vec<Line>) {
    let valid_count = lines
        .iter()
        .filter(|line| {
            let chars = line.password.chars().collect::<Vec<char>>();
            let matches = (
                chars[(line.start - 1) as usize] == line.character,
                chars[(line.end - 1) as usize] == line.character,
            );
            match matches {
                (false, true) => true,
                (true, false) => true,
                _ => false,
            }
        })
        .count();
    println!("part2 valid count: {}", valid_count);
}
