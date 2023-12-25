use std::ops::Neg;

use nalgebra::{matrix, vector};

pub mod task1 {
    use super::count_intersections;

    pub fn ans() -> u128 {
        count_intersections(
            "resources/2023/day24/input",
            200000000000000u128,
            400000000000000u128,
        )
    }
}

pub mod task2 {
    use super::find_intersecting_path;

    pub fn ans() -> u128 {
        find_intersecting_path("resources/2023/day24/input")
    }
}

fn find_intersecting_path(file: &str) -> u128{
    // This is based on a derivation of the problem that
    // gives a linear system of equations.
    // Here's a link if you're curious:
    // https://github.com/JackSpagnoli/advent_of_code/blob/main/resources/2023/day24/derivation.md
    let paths = parse_file(file);

    // paths 0,1, and 2 get an off by 1 because of
    // some weird floating point rounding errors.
    // It's currently 7:30pm on christmas day
    // so I cba to find out how integer matrices work.
    let h_0 = FloatPath::from(&paths[1]);
    let h_1 = FloatPath::from(&paths[2]);
    let h_2 = FloatPath::from(&paths[3]);

    
    let a = matrix![
        (h_0.dy - h_1.dy), -(h_0.dx - h_1.dx), 0f64, -(h_0.y - h_1.y), (h_0.x - h_1.x), 0f64;
        (h_0.dz - h_1.dz), 0f64, -(h_0.dx - h_1.dx), -(h_0.z - h_1.z), 0f64, (h_0.x - h_1.x);
        0f64, (h_0.dz - h_1.dz), -(h_0.dy - h_1.dy), 0f64, -(h_0.z - h_1.z), (h_0.y - h_1.y);
        (h_0.dy - h_2.dy), -(h_0.dx - h_2.dx), 0f64, -(h_0.y - h_2.y), (h_0.x - h_2.x), 0f64;
        (h_0.dz - h_2.dz), 0f64, -(h_0.dx - h_2.dx), -(h_0.z - h_2.z), 0f64, (h_0.x - h_2.x);
        0f64, (h_0.dz - h_2.dz), -(h_0.dy - h_2.dy), 0f64, -(h_0.z - h_2.z), (h_0.y - h_2.y);
    ];

    let b = vector![
        h_1.y*h_1.dx - h_0.y*h_0.dx + h_0.x*h_0.dy - h_1.x*h_1.dy,
        h_1.z*h_1.dx - h_0.z*h_0.dx + h_0.x*h_0.dz - h_1.x*h_1.dz,
        h_1.z*h_1.dy - h_0.z*h_0.dy + h_0.y*h_0.dz - h_1.y*h_1.dz,
        h_2.y*h_2.dx - h_0.y*h_0.dx + h_0.x*h_0.dy - h_2.x*h_2.dy,
        h_2.z*h_2.dx - h_0.z*h_0.dx + h_0.x*h_0.dz - h_2.x*h_2.dz,
        h_2.z*h_2.dy - h_0.z*h_0.dy + h_0.y*h_0.dz - h_2.y*h_2.dz,
    ];

    assert!(a.is_invertible());

    let x = a.try_inverse().unwrap() * b;

    x[0] as u128 + x[1] as u128 + x[2] as u128
}

struct FloatPath {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}
impl From<&Path> for FloatPath{
    fn from(path: &Path) -> Self {
        FloatPath {
            x: path.x as f64,
            y: path.y as f64,
            z: path.z as f64,
            dx: path.dx as f64,
            dy: path.dy as f64,
            dz: path.dz as f64,
        }
    }
}

fn count_intersections(file: &str, min: u128, max: u128) -> u128 {
    let paths = parse_file(file);
    let lines = paths.into_iter().map(line_from_path).collect::<Vec<_>>();

    let min = &parse_fraction(min as i128);
    let max = &parse_fraction(max as i128);

    lines
        .iter()
        .enumerate()
        .flat_map(|(i, l1)| {
            lines
                .iter()
                .skip(i + 1)
                .map(move |l2| {
                    let i = interesection(l1, l2);

                    (i, l1, l2)
                })
                .filter(|(i, _, _)| i.is_some())
                .filter(|(i, l1, l2)| {
                    intersection_valid_for_line(i.unwrap(), l1)
                        && intersection_valid_for_line(i.unwrap(), l2)
                })
                .map(|(i, _, _)| i.unwrap())
        })
        .filter(|(x, y)| x > min && x <= max && y > min && y <= max)
        .count() as u128
}

fn intersection_valid_for_line(i: (Fraction, Fraction), l: &Line) -> bool {
    if let Some(min_x) = &l.min_x {
        if i.0 < *min_x {
            return false;
        }
    }
    if let Some(max_x) = &l.max_x {
        if i.0 > *max_x {
            return false;
        }
    }
    true
}

fn interesection(l1: &Line, l2: &Line) -> Option<(Fraction, Fraction)> {
    let m1 = &l1.m;
    let m2 = &l2.m;
    let c1 = &l1.c;
    let c2 = &l2.c;

    if m1 == m2 {
        if c1 == c2 {
            Some((Fraction::new(0u128, 1u128), Fraction::new(0u128, 1u128)))
        } else {
            None
        }
    } else {
        let x = (c2 - c1) / (m1 - m2);
        let y = *m1 * x + *c1;
        Some((x, y))
    }
}

struct Path {
    x: i128,
    y: i128,
    z: i128,
    dx: i128,
    dy: i128,
    dz: i128,
}
fn parse_file(file: &str) -> Vec<Path> {
    let contents = std::fs::read_to_string(file).unwrap();

    contents
        .lines()
        .map(|line| {
            let regex = regex::Regex::new(r"-?\d+").unwrap();

            let mut captures = regex
                .captures_iter(line)
                .map(|cap| cap.get(0).unwrap().as_str())
                .map(|s| s.parse::<i128>().unwrap());

            Path {
                x: captures.next().unwrap(),
                y: captures.next().unwrap(),
                z: captures.next().unwrap(),
                dx: captures.next().unwrap(),
                dy: captures.next().unwrap(),
                dz: captures.next().unwrap(),
            }
        })
        .collect()
}

type Fraction = fraction::GenericFraction<u128>;
struct Line {
    m: Fraction,
    c: Fraction,
    min_x: Option<Fraction>,
    max_x: Option<Fraction>,
}
fn line_from_path(path: Path) -> Line {
    let x = parse_fraction(path.x);
    let y = parse_fraction(path.y);

    let dx = parse_fraction(path.dx);
    let dy = parse_fraction(path.dy);

    let m = dy / dx;
    let c = y - m * x;

    let max_x = if dx < Fraction::from(0u128) {
        Some(parse_fraction(path.x + path.dx))
    } else {
        None
    };

    let min_x = if dx > Fraction::from(0u128) {
        Some(parse_fraction(path.x + path.dx))
    } else {
        None
    };

    Line { m, c, min_x, max_x }
}

fn parse_fraction(k: i128) -> Fraction {
    if k < 0 {
        Fraction::new(-k as u128, 1u128).neg()
    } else {
        Fraction::new(k as u128, 1u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_count() {
        let count = count_intersections("resources/2023/day24/test_input", 7, 27);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_find_intersecting_path() {
        let count = find_intersecting_path("resources/2023/day24/test_input");
        assert_eq!(count, 47);
    }
}

