use adventofcode2020::utils::file;
use std::collections::HashSet;

fn main() {
    let numbers = file::lines("inputs/day9.txt")
        .iter()
        .map(|num| str::parse::<i64>(num).unwrap())
        .collect();

    part1(&numbers);
    part2(&numbers);
}

fn find_invalid_number(numbers: &Vec<i64>) -> i64 {
    let preamble_length = 25;

    fn is_sum_of_prev(num: i64, slice: &[i64]) -> bool {
        let prev = slice.iter().collect::<HashSet<_>>();
        slice.iter().any(|curr| {
            let pair = num - curr;
            num != pair && prev.contains(&pair)
        })
    }

    numbers
        .windows(preamble_length + 1)
        .map(|window| {
            let num = *window.last().unwrap();
            let slice = &window[0..preamble_length];
            (slice, num)
        })
        .find(|(slice, num)| !is_sum_of_prev(*num, slice))
        .map(|(_, num)| num)
        .unwrap()
}

fn part1(numbers: &Vec<i64>) {
    println!("part 1 {:?}", find_invalid_number(numbers));
}

fn part2(numbers: &Vec<i64>) {
    let invalid_number = find_invalid_number(numbers);

    let mut min = 0;
    let mut max = 0;
    for i in 0..numbers.len() {
        let mut sum = 0i64;
        let mut found = false;
        let nums = numbers
            .iter()
            .take(i)
            .rev()
            .cloned()
            .take_while(|prev| {
                sum += prev;
                if sum == invalid_number {
                    found = true;
                }
                return sum <= invalid_number;
            })
            .collect::<Vec<_>>();
        if found {
            min = *nums.iter().min().unwrap();
            max = *nums.iter().max().unwrap();
            break;
        }
    }

    println!("part 2 {:?}", min + max);
}
