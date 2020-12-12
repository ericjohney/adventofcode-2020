use adventofcode2020::utils::file;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Space {
    Floor,
    Empty,
    Occupied,
}

type Spaces = HashMap<(usize, usize), Space>;

fn main() {
    let input = file::lines("inputs/day11.txt");
    let mut spaces = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        for (j, space_text) in line.chars().enumerate() {
            let space = match space_text {
                'L' => Space::Empty,
                '#' => Space::Occupied,
                _ => Space::Floor,
            };
            spaces.insert((j, i), space);
        }
    }

    part1(&spaces);
    part2(&spaces);
}

fn get_space_bounds(spaces: &Spaces) -> (usize, usize) {
    let keys = spaces.keys();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for (x, y) in keys {
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
    }
    (max_x, max_y)
}

fn part1(input: &Spaces) {
    fn get_adjacent(
        spaces: &Spaces,
        index: &(usize, usize),
        space_size: (usize, usize),
    ) -> Vec<Space> {
        let mut adjacent = Vec::new();
        let x = index.0 as i64;
        let y = index.1 as i64;
        let bounds = (
            cmp::max(x - 1, 0) as usize,
            cmp::max(y - 1, 0) as usize,
            cmp::min(x + 1, space_size.0 as i64) as usize,
            cmp::min(y + 1, space_size.1 as i64) as usize,
        );

        for i in bounds.0..=bounds.2 {
            for j in bounds.1..=bounds.3 {
                // don't this space
                if !(i == x as usize && j == y as usize) {
                    adjacent.push(*spaces.get(&(i, j)).unwrap());
                }
            }
        }
        adjacent
    }

    fn update_spaces(spaces: &mut Spaces, space_size: (usize, usize)) -> i64 {
        let mut updated_count = 0;
        let read_only_spaces = spaces.clone();
        for (key, space) in spaces {
            let adjacent_occupied_count = get_adjacent(&read_only_spaces, key, space_size)
                .iter()
                .filter(|space| **space == Space::Occupied)
                .count();
            match space {
                Space::Empty => {
                    if adjacent_occupied_count == 0 {
                        *space = Space::Occupied;
                        updated_count += 1;
                    }
                }
                Space::Occupied => {
                    if adjacent_occupied_count >= 4 {
                        *space = Space::Empty;
                        updated_count += 1;
                    }
                }
                Space::Floor => {}
            }
        }
        updated_count
    }

    let mut spaces = input.clone();
    let space_size = get_space_bounds(&spaces);

    loop {
        let updated_count = update_spaces(&mut spaces, space_size);
        if updated_count == 0 {
            break;
        }
    }
    let occupied_seats_count = spaces.values().filter(|s| **s == Space::Occupied).count();

    println!("part 1 {:?}", occupied_seats_count);
}

fn part2(input: &Spaces) {
    fn get_adjacent(
        spaces: &Spaces,
        index: &(usize, usize),
        space_size: (usize, usize),
    ) -> Vec<Space> {
        let mut adjacent = Vec::new();
        let x = index.0;
        let y = index.1;

        let to_right = cmp::min(x, space_size.0)..=space_size.0;
        let to_left = (0..=cmp::max(x, 0)).rev();
        let to_down = cmp::min(y, space_size.1)..=space_size.1;
        let to_up = (0..=cmp::max(y, 0)).rev();

        let mut append = |iter: Box<dyn Iterator<Item = (usize, usize)>>| {
            let get_space = |(i, j)| {
                // don't count the current space
                if (i, j) == *index {
                    return None;
                }

                let space = spaces.get(&(i, j))?;
                if *space == Space::Floor {
                    return None;
                }
                Some(*space)
            };
            let space = iter.filter_map(get_space).nth(0)?;
            adjacent.push(space);
            Some(space)
        };

        append(Box::new(to_right.clone().map(|i| (i, y))));
        append(Box::new(to_left.clone().map(|i| (i, y))));
        append(Box::new(to_up.clone().map(|i| (x, i))));
        append(Box::new(to_down.clone().map(|i| (x, i))));
        append(Box::new(to_right.clone().zip(to_up.clone())));
        append(Box::new(to_right.clone().zip(to_down.clone())));
        append(Box::new(to_left.clone().zip(to_up.clone())));
        append(Box::new(to_left.clone().zip(to_down.clone())));

        adjacent
    }

    fn update_spaces(spaces: &mut Spaces, space_size: (usize, usize)) -> i64 {
        let mut updated_count = 0;
        let read_only_spaces = spaces.clone();
        for (key, space) in spaces {
            let adjacent_occupied_count = get_adjacent(&read_only_spaces, key, space_size)
                .iter()
                .filter(|space| **space == Space::Occupied)
                .count();
            match space {
                Space::Empty => {
                    if adjacent_occupied_count == 0 {
                        *space = Space::Occupied;
                        updated_count += 1;
                    }
                }
                Space::Occupied => {
                    if adjacent_occupied_count >= 5 {
                        *space = Space::Empty;
                        updated_count += 1;
                    }
                }
                Space::Floor => {}
            }
        }
        updated_count
    }

    let mut spaces = input.clone();
    let space_size = get_space_bounds(&spaces);

    loop {
        let updated_count = update_spaces(&mut spaces, space_size);
        if updated_count == 0 {
            break;
        }
    }
    let occupied_seats_count = spaces.values().filter(|s| **s == Space::Occupied).count();

    println!("part 2 {:?}", occupied_seats_count);
}
