use std::{fs, cmp::{max, min}};

use regex::Regex;

pub mod task1 {
    use super::beaconless_spaces;

    pub fn ans() -> u128 {
        beaconless_spaces("resources/2022/day15/input", 2000000) as u128
    }
}

pub mod task2 {
    use super::tuning_frequency;

    pub fn ans() -> u128 {
        tuning_frequency("resources/2022/day15/input", 4_000_000) as u128
    }
}

fn tuning_frequency(file: &str, range: isize) -> isize {
    let (sensor_coordinates, _, _) = sensor_coordinate_pairs(file);

    for i in 0..sensor_coordinates.len() {
        let ((s_y, s_x), distance) = sensor_coordinates[i];

        for step in 0..distance + 2 {
            let neighbour_coordinates: Vec<(isize, isize)> = vec![
                (s_y + step - distance - 1, s_x + step),
                (s_y + step, s_x + distance + 1 - step),
                (s_y - step + distance + 1, s_x - step),
                (s_y - step, s_x + step - distance - 1)
            ];
            for (y, x) in neighbour_coordinates {
                if in_range((y, x), range) && !has_sensor_in_range((y, x), &sensor_coordinates) {
                    return 4_000_000 * x + y;
                }
            }
        }
    }
    return 0;
}

fn in_range(coordinates: (isize, isize), range: isize) -> bool {
    coordinates.0 >= 0 && coordinates.0 <= range && coordinates.1 >= 0 && coordinates.1 <= range
}

fn beaconless_spaces(file: &str, row: usize) -> usize {
    let (sensor_coordinates, beacon_coordinates, (min_x, max_x)) = sensor_coordinate_pairs(file);

    let mut spaces: usize = 0;
    for x in min_x..max_x + 1 {
        if has_sensor_in_range((row as isize, x), &sensor_coordinates) {
            spaces += 1;
        }
    }

    for i in 0..beacon_coordinates.len() {
        if beacon_coordinates[i].0 == (row as isize) {
            spaces -= 1;
        }
    }

    return spaces;
}

fn sensor_coordinate_pairs(
    file: &str
) -> (Vec<((isize, isize), isize)>, Vec<(isize, isize)>, (isize, isize)) {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let regex = Regex::new(
        r"Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>\d+)"
    ).unwrap();

    let mut sensor_coordinates: Vec<((isize, isize), isize)> = vec!();
    let mut beacon_coordinates: Vec<(isize, isize)> = vec!();

    let mut min_x: isize = isize::MAX;
    let mut max_x: isize = 0;
    let mut max_distance: isize = 0;
    for capture in regex.captures_iter(&contents) {
        let coords: ((isize, isize), (isize, isize)) = (
            (capture["s_y"].parse::<isize>().unwrap(), capture["s_x"].parse::<isize>().unwrap()),
            (capture["b_y"].parse::<isize>().unwrap(), capture["b_x"].parse::<isize>().unwrap()),
        );

        let distance = l1_distance(coords.0, coords.1);
        max_distance = max(max_distance, distance);

        min_x = min(min_x, min(coords.0.1, coords.1.1));
        max_x = max(max_x, max(coords.0.1, coords.1.1));

        sensor_coordinates.push((coords.0, distance));
        if !beacon_coordinates.contains(&coords.1) {
            beacon_coordinates.push(coords.1);
        }
    }

    return (sensor_coordinates, beacon_coordinates, (min_x - max_distance, max_x + max_distance));
}

fn l1_distance((y_1, x_1): (isize, isize), (y_2, x_2): (isize, isize)) -> isize {
    let dy: isize = y_1 - y_2;
    let dx: isize = x_1 - x_2;
    return max(dy, -dy) + max(dx, -dx);
}

fn has_sensor_in_range(
    (y, x): (isize, isize),
    sensor_coordinates: &Vec<((isize, isize), isize)>
) -> bool {
    for i in 0..sensor_coordinates.len() {
        let (coordinates, distance) = sensor_coordinates[i];
        if l1_distance(coordinates, (y, x)) <= distance {
            return true;
        }
    }
    return false;
}