use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub mod task1 {
    use super::count_antinodes;

    pub fn ans() -> u128 {
        count_antinodes("resources/2024/day08/input.txt", true)
    }
}

pub mod task2 {
    use super::count_antinodes;

    pub fn ans() -> u128 {
        count_antinodes("resources/2024/day08/input.txt", false)
    }
}

fn count_antinodes(file: &str, single: bool) -> u128 {
    let content = fs::read_to_string(file).unwrap();

    let map_height = content.lines().count() as isize;
    let map_width = content.lines().next().unwrap().len() as isize;

    let content = fs::read_to_string(file).unwrap();

    let mut antenna: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    content
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (x as isize, y as isize, c))
        })
        .filter(|(_, _, c)| c != &'.')
        .for_each(|(x, y, c)| {
            if !antenna.contains_key(&c) {
                antenna.insert(c, Vec::new());
            }

            let c_antenna = antenna.get_mut(&c).unwrap();
            c_antenna.push((x, y));
        });

    let antinodes = antenna.values().fold(HashSet::new(), |acc, v| {
        add_antinodes(acc, v, map_width, map_height, single)
    });

    antinodes.into_iter().count() as u128
}

fn add_antinodes(
    antinodes: HashSet<(isize, isize)>,
    antenna: &Vec<(isize, isize)>,
    map_width: isize,
    map_height: isize,
    single: bool,
) -> HashSet<(isize, isize)> {
    let pairs = (0..antenna.len() - 1)
        .flat_map(|i| (i + 1..antenna.len()).map(move |j| (i, j)))
        .map(|(i, j)| (&antenna[i], &antenna[j]));

    pairs.fold(antinodes, |acc, pair| {
        add_antinodes_for_pair(acc, pair, map_width, map_height, single)
    })
}

fn add_antinodes_for_pair(
    mut antinodes: HashSet<(isize, isize)>,
    pair: (&(isize, isize), &(isize, isize)),
    map_width: isize,
    map_height: isize,
    single: bool,
) -> HashSet<(isize, isize)> {
    let ((x1, y1), (x2, y2)) = pair;

    let dx = x2 - x1;
    let dy = y2 - y1;

    let disp_pos = |i| {
        let x = x1 + i * dx;
        let y = y1 + i * dy;

        (x, y)
    };

    let is_valid = |(x, y)| x >= 0 && y >= 0 && x < map_width && y < map_height;

    if single {
        let node_1 = (x1 - dx, y1 - dy);
        let node_2 = (x2 + dx, y2 + dy);

        if is_valid(node_1) {
            antinodes.insert(node_1);
        }

        if is_valid(node_2) {
            antinodes.insert(node_2);
        }

        return antinodes;
    }

    let positives = (0..).map(|i| disp_pos(i)).take_while(|pos| is_valid(*pos));
    let negatives = (1..).map(|i| disp_pos(-i)).take_while(|pos| is_valid(*pos));

    positives.chain(negatives).for_each(|pos| {
        antinodes.insert(pos);
    });

    antinodes
}

#[cfg(test)]
mod tests {
    use super::count_antinodes;

    #[test]
    fn test_count_antinodes() {
        assert_eq!(
            count_antinodes("resources/2024/day08/test_input.txt", true),
            14
        );
    }

    #[test]
    fn test_count_antinodes_multiples() {
        assert_eq!(
            count_antinodes("resources/2024/day08/test_input.txt", false),
            34
        );
    }
}
