use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub mod task1 {
    use super::count_antinodes;

    pub fn ans() -> u128 {
        count_antinodes("resources/2024/day08/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

fn count_antinodes(file: &str) -> u128 {
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

    let antinodes = antenna
        .values()
        .fold(HashSet::new(), |acc, v| add_antinodes(acc, v));

    antinodes
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < map_width && *y < map_height)
        .count() as u128
}

fn add_antinodes(
    mut antinodes: HashSet<(isize, isize)>,
    antenna: &Vec<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    let pair_indices =
        (0..antenna.len() - 1).flat_map(|i| (i + 1..antenna.len()).map(move |j| (i, j)));

    pair_indices.for_each(|(i, j)| {
        let (x1, y1) = antenna[i];
        let (x2, y2) = antenna[j];

        let dx = x2 - x1;
        let dy = y2 - y1;

        let node_1_x = x1 - dx;
        let node_1_y = y1 - dy;

        let node_2_x = x2 + dx;
        let node_2_y = y2 + dy;

        antinodes.insert((node_1_x, node_1_y));
        antinodes.insert((node_2_x, node_2_y));
    });

    antinodes
}

#[cfg(test)]
mod tests {
    use super::count_antinodes;

    #[test]
    fn test_count_antinodes() {
        assert_eq!(count_antinodes("resources/2024/day08/test_input.txt"), 14);
    }

    #[bench]
    fn bench_count_antinodes(b: &mut crate::Bencher) {
        b.iter(|| count_antinodes("resources/2024/day08/test_input.txt"));
    }
}
