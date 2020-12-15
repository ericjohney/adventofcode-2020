use adventofcode2020::utils::file;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let schedule = file::lines("inputs/day13.txt");
    let time = str::parse::<i64>(&schedule[0]).unwrap();
    let busses = schedule[1].split(",").collect::<Vec<_>>();

    part1(time, &busses);
    part2(&busses);
}

fn part1(time: i64, schedule: &Vec<&str>) {
    let busses = schedule
        .iter()
        .filter(|bus| **bus != "x")
        .map(|bus| str::parse::<i64>(bus).unwrap())
        .collect::<Vec<_>>();

    let wait_times = busses
        .iter()
        .map(|bus| {
            let missed_by = time % bus;
            let wait_time = (bus - missed_by) % bus;
            return (*bus, wait_time);
        })
        .collect::<HashMap<i64, i64>>();

    let shortest_wait = wait_times.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    println!("part 1 {:?}", shortest_wait.0 * shortest_wait.1);
}

fn part2(schedule: &Vec<&str>) {
    fn inv_mod(x: i64, p: i64) -> i64 {
        // p must be prime
        (0..p - 2).fold(1, |o, _| (o * x) % p)
    }

    let busses: Vec<(i64, i64)> = schedule
        .iter()
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|b| (i as i64, b)))
        .collect();

    let prod: i64 = busses.iter().map(|(_, b)| b).product();

    let result = busses
        .iter()
        .map(|&(a, b)| -a * (prod / b) * inv_mod(prod / b, b))
        .sum::<i64>()
        .rem_euclid(prod);

    println!("part 2 {:?}", result);
}
