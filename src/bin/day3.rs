use adventofcode2019::file_utils;

fn main() {
    let map = file_utils::lines("inputs/day3.txt");

    part1(&map);
    part2(&map);
}

fn count_trees(map: &Vec<String>, x_inc: usize, y_inc: usize) -> i64 {
    let mut x = 0;
    map.iter()
        .step_by(y_inc)
        .map(|line| {
            let value = line.chars().nth(x).unwrap();
            x = (x + x_inc) % line.len();
            match value {
                '#' => 1,
                _ => 0,
            }
        })
        .sum()
}

fn part1(map: &Vec<String>) {
    let tree_count = count_trees(map, 3, 1);
    println!("part1 {}", tree_count);
}

fn part2(map: &Vec<String>) {
    let moves = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: i64 = moves.iter().map(|m| count_trees(map, m.0, m.1)).product();
    println!("part2 {}", product);
}
