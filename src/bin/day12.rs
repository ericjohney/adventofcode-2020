use adventofcode2020::utils::file;

fn main() {
	let instructions = file::lines("inputs/day12.txt")
		.iter()
		.map(|input| {
			let action = input.chars().nth(0).unwrap();
			let num = str::parse::<i64>(&input[1..]).unwrap();
			(action, num)
		})
		.collect::<Vec<_>>();

	part1(&instructions);
	part2(&instructions);
}

fn part1(instructions: &Vec<(char, i64)>) {
	let mut current_angle = 0;
	let mut position = (0, 0);
	for (action, num) in instructions {
		current_angle = match action {
			'L' => (current_angle + 360) - num,
			'R' => (current_angle + num) % 360,
			_ => current_angle,
		};
		position = match action {
			'N' => (position.0, position.1 - num),
			'S' => (position.0, position.1 + num),
			'E' => (position.0 + num, position.1),
			'W' => (position.0 - num, position.1),
			'F' => {
				let float_num = *num as f64;
				let sin = (current_angle as f64).to_radians().sin();
				let cos = (current_angle as f64).to_radians().cos();
				(
					position.0 + (cos * float_num) as i64,
					position.1 + (sin * float_num) as i64,
				)
			}
			_ => position,
		};
	}
	println!("part1 {:?}", position.0.abs() + position.1.abs());
}

fn part2(instructions: &Vec<(char, i64)>) {
	let mut position = (0, 0);
	let mut waypoint = (10, 1);
	for (action, int_num) in instructions {
		let num = *int_num as f64;
		let waypoint_distance: (i64, i64) = ((waypoint.0 - position.0), (waypoint.1 - position.1));
		let mut current_angle = (waypoint_distance.1 as f64 / waypoint_distance.0 as f64)
			.atan()
			.to_degrees();
		if waypoint_distance.0 < 0 {
			current_angle = (current_angle + 180.0) % 360.0;
		}
		waypoint = match action {
			'L' => {
				let new_angle = current_angle.to_radians() + num.to_radians();
				let sin = new_angle.sin();
				let cos = new_angle.cos();
				let distance = (waypoint_distance.0.abs() as f64).hypot(waypoint_distance.1.abs() as f64);
				(
					position.0 + (cos * distance).round() as i64,
					position.1 + (sin * distance).round() as i64,
				)
			}
			'R' => {
				let new_angle = current_angle.to_radians() - num.to_radians();
				let sin = new_angle.sin();
				let cos = new_angle.cos();
				let distance = (waypoint_distance.0.abs() as f64).hypot(waypoint_distance.1.abs() as f64);
				(
					position.0 + (cos * distance).round() as i64,
					position.1 + (sin * distance).round() as i64,
				)
			}
			_ => waypoint,
		};
		waypoint = match action {
			'N' => (waypoint.0, waypoint.1 + int_num),
			'S' => (waypoint.0, waypoint.1 - int_num),
			'E' => (waypoint.0 + int_num, waypoint.1),
			'W' => (waypoint.0 - int_num, waypoint.1),
			_ => waypoint,
		};
		match action {
			'F' => {
				for _ in 0..*int_num {
					position = waypoint;
					waypoint = (
						(waypoint.0 + waypoint_distance.0),
						(waypoint.1 + waypoint_distance.1),
					);
				}
			}
			_ => {}
		};
	}
	println!("part2 {:?}", position.0.abs() + position.1.abs());
}
