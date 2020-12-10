use adventofcode2020::utils::file;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let mut adapters = file::lines("inputs/day10.txt")
        .iter()
        .map(|num| str::parse::<i64>(num).unwrap())
        .collect::<Vec<_>>();

    adapters.sort();

    part1(&adapters);
    part2(&adapters);
}

fn part1(adapters: &Vec<i64>) {
    let mut jolt_counts: HashMap<i64, i64> = HashMap::new();
    let mut last_adapter = 0;
    for adapter in adapters {
        let diff = adapter - last_adapter;
        let count = *jolt_counts.entry(diff).or_insert(0i64);
        jolt_counts.insert(diff, count + 1);
        last_adapter = *adapter;
    }
    println!(
        "part 1 {:?}",
        jolt_counts.get(&1).unwrap() * (jolt_counts.get(&3).unwrap() + 1)
    );
}

fn part2(input: &Vec<i64>) {
    let mut adapters = input.iter().cloned().collect::<Vec<_>>();
    adapters.insert(0, 0);
    adapters.push(input.last().unwrap() + 3);

    let mut options_count = vec![1];
    for i in 1..adapters.len() {
        let mut count = 0u64;
        let start = cmp::max(i as i64 - 3, 0) as usize;
        for j in start..i {
            let is_optional = adapters[j] + 3 >= adapters[i];
            if is_optional {
                count += options_count[j];
            }
        }
        options_count.push(count);
    }

    println!("part 2 {:?}", options_count.last().unwrap());
}
