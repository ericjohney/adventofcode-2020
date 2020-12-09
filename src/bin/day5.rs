use adventofcode2020::utils::file;

fn main() {
    let seats = file::lines("inputs/day5.txt")
        .iter()
        .map(|location| get_seat(location))
        .collect();

    part1(&seats);
    part2(&seats);
}

fn get_seat(location: &str) -> (i64, i64) {
    let mut rows = 0..127;
    let mut aisles = 0..7;
    for instruction in location.chars() {
        println!("instruction {}", instruction);
        match instruction {
            'F' => {
                rows = rows.start..(((rows.end - rows.start) / 2) + rows.start);
            }
            'B' => {
                rows = (((rows.end - rows.start) / 2) + rows.start + 1)..(rows.end);
            }
            'L' => {
                aisles = aisles.start..(((aisles.end - aisles.start) / 2) + aisles.start);
            }
            'R' => {
                aisles = (((aisles.end - aisles.start) / 2) + aisles.start + 1)..(aisles.end);
            }
            _ => {}
        }
    }
    return (rows.start, aisles.start);
}

fn part1(seats: &Vec<(i64, i64)>) {
    let ids = seats
        .iter()
        .map(|seat| seat.0 * 8 + seat.1)
        .collect::<Vec<i64>>();
    let max_id = ids.iter().max().unwrap();
    println!("part1 max id {}", max_id);
}

fn part2(seats: &Vec<(i64, i64)>) {
    let mut ids = seats
        .iter()
        .map(|seat| seat.0 * 8 + seat.1)
        .collect::<Vec<i64>>();
    ids.sort();
    let mut last_id = ids.pop().unwrap();
    let mut missing_id = 0;
    for id in ids {
        if id > last_id + 1 {
            missing_id = id - 1;
            break;
        }
        last_id = id;
    }
    println!("part2 missing id {}", missing_id);
}
