use adventofcode2020::file_utils;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

fn main() {
    let points = file_utils::parse_lines(
        "inputs/2018/day10.txt",
        r"(?m)<(.*?),\s?(.*?)>.*?<(.*?),\s?(.*?)>",
    )
    .iter()
    .map(|c| Point {
        x: str::parse::<i64>(&c[1].trim()).unwrap(),
        y: str::parse::<i64>(&c[2].trim()).unwrap(),
        velocity_x: str::parse::<i64>(&c[3].trim()).unwrap(),
        velocity_y: str::parse::<i64>(&c[4].trim()).unwrap(),
    })
    .collect();

    part1(&points);
    // part2(&lines);
}

// fn count_trees(map: &Vec<String>, x_inc: usize, y_inc: usize) -> i64 {
//     let mut x = 0;
//     map.iter()
//         .step_by(y_inc)
//         .map(|line| {
//             let value = line.chars().nth(x).unwrap();
//             x = (x + x_inc) % line.len();
//             match value {
//                 '#' => 1,
//                 _ => 0,
//             }
//         })
//         .sum()
// }

fn part1(input_points: &Vec<Point>) {
    let mut points: Vec<Point> = input_points.iter().map(|point| *point).collect();
    let mut min_area = None::<i64>;
    let mut point_hash = HashSet::new();
    for iterations in 0..20000 {
        let new_points: Vec<Point> = points
            .iter()
            .map(|point| Point {
                x: point.x + point.velocity_x,
                y: point.y + point.velocity_y,
                velocity_x: point.velocity_x,
                velocity_y: point.velocity_y,
            })
            .collect();

        let max_x = new_points.iter().map(|c| c.x).max().unwrap();
        let max_y = new_points.iter().map(|c| c.y).max().unwrap();
        let min_x = new_points.iter().map(|c| c.x).min().unwrap();
        let min_y = new_points.iter().map(|c| c.y).min().unwrap();

        let area = (max_x - min_x) * (max_y - min_y);
        if min_area.is_none() || area < min_area.unwrap() {
            min_area = Some(area);
            points = new_points;
            point_hash = HashSet::new();
            points.iter().for_each(|point| {
                point_hash.insert((point.x, point.y));
            });
        } else {
            break;
        }
    }

    let max_x = points.iter().map(|c| c.x).max().unwrap();
    let max_y = points.iter().map(|c| c.y).max().unwrap();
    let min_x = points.iter().map(|c| c.x).min().unwrap();
    let min_y = points.iter().map(|c| c.y).min().unwrap();

    let mut output = "".to_string();
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            if point_hash.contains(&(x, y)) {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }

    println!("{}", output);
}

// fn part2(map: &Vec<String>) {
//     let moves = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
//     let product: i64 = moves.iter().map(|m| count_trees(map, m.0, m.1)).product();
//     println!("part2 {}", product);
// }
