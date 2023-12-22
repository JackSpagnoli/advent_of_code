use std::fs;

pub mod task1 {
    use super::calculate_max_sand_volume;

    pub fn ans() -> u128 {
        calculate_max_sand_volume("resources/2022/day14/input", false) as u128
    }
}

pub mod task2 {
    use super::calculate_max_sand_volume;

    pub fn ans() -> u128 {
        calculate_max_sand_volume("resources/2022/day14/input", true) as u128
    }
}

fn calculate_max_sand_volume(file: &str, solid_floor: bool) -> usize {
    let (mut map, source) = generate_rock_map(file, solid_floor);

    let mut particles: usize = 0;
    while simulate_sand_particle(&mut map, source) {
        particles += 1;
    }

    particles
}

fn simulate_sand_particle(map: &mut [Vec<usize>], source: (usize, usize)) -> bool {
    let mut pos = source;
    loop {
        if pos.0 + 1 >= map.len() || map[pos.0][pos.1] == 2 {
            return false;
        }
        if map[pos.0 + 1][pos.1] == 0 {
            pos.0 += 1;
        } else if map[pos.0 + 1][pos.1 - 1] == 0 {
            pos.0 += 1;
            pos.1 -= 1;
        } else if map[pos.0 + 1][pos.1 + 1] == 0 {
            pos.0 += 1;
            pos.1 += 1;
        } else {
            map[pos.0][pos.1] = 2;
            return true;
        }
    }
}

fn generate_rock_map(file: &str, solid_floor: bool) -> (Vec<Vec<usize>>, (usize, usize)) {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let lines = contents.lines();

    let mut all_rocks: Vec<(usize, usize)> = vec![];

    let mut minx: usize = 2_000_000;
    let mut maxx: usize = 0;
    let mut maxy: usize = 0;
    for line in lines {
        let rocks: Vec<(usize, usize)> = parse_path_line(line);
        for rock in rocks {
            all_rocks.push(rock);
            if rock.0 > maxy {
                maxy = rock.0;
            }
            if rock.1 > maxx {
                maxx = rock.1;
            }
            if rock.1 < minx {
                minx = rock.1;
            }
        }
    }

    let width: usize;
    let height: usize;
    if !solid_floor {
        width = maxx - minx + 1;
        height = maxy;
    } else {
        height = maxy;
        width = maxx - minx + 1 + 2 * height;
    }

    let mut map: Vec<Vec<usize>> = vec![];
    for _ in 0..height + 3 {
        map.push(
            std::iter::repeat(0)
                .take(width + 2)
                .collect::<Vec<usize>>(),
        );
    }

    for rock in all_rocks {
        if !solid_floor {
            map[rock.0][rock.1 - minx + 1] = 1;
        } else {
            map[rock.0][rock.1 - minx + 1 + height] = 1;
        }
    }

    if solid_floor {
        for i in 0..width {
            map[height + 2][i] = 1;
        }
    }

    if !solid_floor {
        (map, (0, 500 - minx + 1))
    } else {
        (map, (0, 500 - minx + 1 + height))
    }
}

fn parse_path_line(line: &str) -> Vec<(usize, usize)> {
    let corners = extract_coordinates(line);

    let mut rocks: Vec<(usize, usize)> = vec![];

    for i in 0..corners.len() - 1 {
        let (dy, dx) = (
            (corners[i + 1].0 as isize) - (corners[i].0 as isize),
            (corners[i + 1].1 as isize) - (corners[i].1 as isize),
        );
        let new_rocks: Vec<(usize, usize)>;
        if dx != 0 {
            if dx < 0 {
                let iter = corners[i + 1].1 + 1..corners[i].1 + 1;
                new_rocks = iter.rev().map(|x| (corners[i + 1].0, x)).collect();
            } else {
                let iter = corners[i].1..corners[i + 1].1;
                new_rocks = iter.map(|x| (corners[i + 1].0, x)).collect();
            }
        } else if dy < 0 {
            let iter = corners[i + 1].0 + 1..corners[i].0 + 1;
            new_rocks = iter.rev().map(|y| (y, corners[i + 1].1)).collect();
        } else {
            let iter = corners[i].0..corners[i + 1].0;
            new_rocks = iter.map(|y| (y, corners[i + 1].1)).collect();
        }
        for rock in new_rocks {
            rocks.push(rock);
        }
    }
    rocks.push((corners[corners.len() - 1].0, corners[corners.len() - 1].1));
    rocks
}

fn extract_coordinates(line: &str) -> Vec<(usize, usize)> {
    let mut all_coordinates: Vec<(usize, usize)> = vec![];
    for coordinate in line.split(" -> ") {
        let mut split = coordinate.split(',');
        let mut y_x: (usize, usize) = (0, 0);
        y_x.1 = split.next().unwrap().parse::<usize>().unwrap();
        y_x.0 = split.next().unwrap().parse::<usize>().unwrap();
        all_coordinates.push(y_x);
    }
    all_coordinates
}
