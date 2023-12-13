use std::cmp::min;

pub mod task1 {
    use super::sum_lines_of_reflection;

    pub fn ans() -> u128 {
        sum_lines_of_reflection("resources/2023/day13/input")
    }
}

pub mod task2 {
    use super::sum_new_lines_of_reflection;

    pub fn ans() -> u128 {
        sum_new_lines_of_reflection("resources/2023/day13/input")
    }
}

fn sum_new_lines_of_reflection(file: &str) -> u128 {
    let patterns = parse_file(file);

    patterns
        .iter()
        .map(|pattern| (pattern, find_reflection_lines(pattern)[0]))
        .map(|(pattern, initial_reflection)| {
            (find_patterns_with_smudge(pattern), initial_reflection)
        })
        .map(|(patterns, initial_reflection)| {
            find_valid_reflection(patterns, initial_reflection)
        })
        .map(|reflection| match reflection {
            Reflection::Vertical(x) => x,
            Reflection::Horizontal(x) => 100 * x
        })
        .sum::<usize>() as u128
}

fn find_valid_reflection(patterns: Vec<Pattern>, initial_reflection: Reflection) -> Reflection {
    patterns
        .iter()
        .map(|pattern| (pattern, find_reflection_lines(pattern)))
        .map(|(pattern, reflections)| {
            (
                pattern,
                reflections
                    .into_iter()
                    .filter(|reflection| reflection != &initial_reflection)
                    .collect::<Vec<Reflection>>(),
            )
        })
        .find(|(_, reflections)| !reflections.is_empty())
        .unwrap()
        .1[0]
}

fn find_patterns_with_smudge(pattern: &Pattern) -> Vec<Pattern> {
    let y_range = 0..pattern.len();
    let x_range = 0..pattern[0].len();

    y_range
        .flat_map(|y| x_range.clone().map(move |x| (y, x)))
        .map(|(y, x)| {
            let mut pattern = pattern.clone();
            match pattern[y][x] {
                Tile::Ash => {
                    pattern[y][x] = Tile::Rock;
                    pattern
                }
                Tile::Rock => {
                    pattern[y][x] = Tile::Ash;
                    pattern
                }
            }
        })
        .collect::<Vec<Pattern>>()
}

fn sum_lines_of_reflection(file: &str) -> u128 {
    let patterns = parse_file(file);

    patterns
        .iter()
        .map(|pattern| find_reflection_lines(pattern)[0])
        .map(|reflection| match reflection {
            Reflection::Vertical(x) => x,
            Reflection::Horizontal(x) => 100 * x
        })
        .sum::<usize>() as u128
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}
fn find_reflection_lines(pattern: &Pattern) -> Vec<Reflection> {
    let verticals = (1..pattern[0].len())
        .filter(|x| is_mirror(pattern, *x))
        .map(Reflection::Vertical);

    let transposed_pattern = transpose(pattern);

    let horizontals = (1..transposed_pattern[0].len())
        .filter(|y| is_mirror(&transposed_pattern, *y))
        .map(Reflection::Horizontal);

    verticals.chain(horizontals).collect()
}

fn transpose(pattern: &Pattern) -> Pattern {
    let mut transposed_pattern = vec![vec![Tile::Ash; pattern.len()]; pattern[0].len()];

    pattern.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, tile)| {
            transposed_pattern[i][j] = *tile;
        })
    });

    transposed_pattern
}

fn is_mirror(pattern: &Pattern, x: usize) -> bool {
    // println!("Checking x = {}", x);

    let reflection_length = get_reflection_length(x, pattern[0].len());
    // println!("Reflection length: {}", reflection_length);

    pattern.iter().all(|row| {
        let left_side = row[x - reflection_length..x].iter();
        let right_side = row[x..x + reflection_length].iter();

        // println!("Left side: {:?}\nRight side: {:?}\n", left_side, right_side);

        left_side.eq(right_side.rev())
    })
}

fn get_reflection_length(mirror: usize, length: usize) -> usize {
    min(length - mirror, mirror)
}

type Patterns = Vec<Pattern>;
type Pattern = Vec<Vec<Tile>>;
#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}
fn parse_file(file: &str) -> Patterns {
    let contents = std::fs::read_to_string(file).unwrap();

    contents.split("\n\n").map(parse_pattern).collect()
}

fn parse_pattern(pattern: &str) -> Pattern {
    pattern.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars()
        .map(|c| match c {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => panic!("Unknown tile"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Tile::*;
    use super::*;

    #[test]
    fn test_parse_file() {
        let patterns = parse_file("resources/2023/day13/test_input");

        let exected_pattern_1 = vec![
            vec![Rock, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
            vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
            vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
            vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
            vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
            vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
            vec![Rock, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
        ];

        let expected_pattern_2 = vec![
            vec![Rock, Ash, Ash, Ash, Rock, Rock, Ash, Ash, Rock],
            vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
            vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
            vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
            vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
            vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
            vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
        ];

        assert_eq!(patterns[0], exected_pattern_1);
        assert_eq!(patterns[1], expected_pattern_2);
    }

    #[test]
    fn test_is_mirror() {
        let patterns = parse_file("resources/2023/day13/test_input");

        let pattern = &patterns[0];
        assert!(is_mirror(pattern, 5));
        assert!(!is_mirror(pattern, 6));
    }

    #[test]
    fn test_find_reflection() {
        let mut patterns = parse_file("resources/2023/day13/test_input").into_iter();

        let pattern = patterns.next().unwrap();
        assert_eq!(find_reflection_lines(&pattern)[0], Reflection::Vertical(5));

        let pattern = patterns.next().unwrap();
        assert_eq!(find_reflection_lines(&pattern)[0], Reflection::Horizontal(4));
    }

    #[test]
    fn test_find_reflection_2() {
        let mut patterns = parse_file("resources/2023/day13/test_input2").into_iter();

        let pattern = patterns.next().unwrap();
        assert_eq!(find_reflection_lines(&pattern)[0], Reflection::Vertical(1));

        let pattern = patterns.next().unwrap();
        assert_eq!(find_reflection_lines(&pattern)[0], Reflection::Vertical(10));
    }

    #[test]
    fn test_sum_lines_of_reflection() {
        assert_eq!(
            sum_lines_of_reflection("resources/2023/day13/test_input"),
            405
        );
    }

    #[test]
    fn test_sum_new_lines_of_reflection() {
        assert_eq!(
            sum_new_lines_of_reflection("resources/2023/day13/test_input"),
            400
        );
    }
}
