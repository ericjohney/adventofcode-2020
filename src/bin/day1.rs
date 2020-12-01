use std::collections::HashSet;

fn main() {
    // let text = fs::read_to_string("inputs/day1.txt").unwrap();
    let text = include_str!("../../inputs/day1.txt");
    let expenses = text
        .split_whitespace()
        .map(|s| str::parse::<i64>(s).unwrap())
        .collect();

    part1(&expenses);
    part2(&expenses);
}

fn part1(expenses: &Vec<i64>) {
    let find_pair = || {
        let mut seen = HashSet::new();
        let target = 2020;
        let mut found = None;

        for i in 0..expenses.len() {
            let current = expenses[i] as i64;
            seen.insert(current);
            let pair = target - current;
            if seen.contains(&pair) {
                return Some((current, pair));
            }
        }

        return None;
    };

    let found = find_pair().unwrap();
    let sum = found.0 * found.1;
    println!("part1: {}", sum);
}

fn part2(expenses: &Vec<i64>) {
    let find_pair = || {
        let mut seen = HashSet::new();
        let target = 2020;

        for i in 0..expenses.len() {
            let current = expenses[i] as i64;
            seen.insert(current);

            for j in 0..seen.len() {
                let inner_current = expenses[j] as i64;
                let pair = target - current - inner_current;

                if seen.contains(&pair) {
                    return Some((inner_current, current, pair));
                }
            }
        }

        return None;
    };

    let found = find_pair().unwrap();
    let sum = found.0 * found.1 * found.2;
    println!("part2: {}", sum);
}
