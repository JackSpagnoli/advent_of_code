use std::collections::HashMap;

pub mod task1 {
    use super::number_possible_combinations;

    pub fn ans() -> u128 {
        number_possible_combinations("resources/2023/day12/input")
    }
}

pub mod task2 {
    use super::number_unfolded_possible_combinations;

    pub fn ans() -> u128 {
        number_unfolded_possible_combinations("resources/2023/day12/input")
    }
}

fn number_possible_combinations(file: &str) -> u128 {
    let rows = parse_file(file, false);

    recurse_with_memory(rows)
}

fn number_unfolded_possible_combinations(file: &str) -> u128 {
    let rows = parse_file(file, true);

    recurse_with_memory(rows)
}

fn recurse_with_memory(rows: Vec<Row>) -> u128 {
    let mut memory: HashMap<Row, u128> = HashMap::new();

    rows.into_iter()
        .map(|row| recursive_solve(row, &mut memory))
        .sum()
}

fn unfold_row(row: Row) -> Row {
    let (map, contiguous) = row;

    let new_map = [&map; 5].map(|s| s.to_string()).join("?");
    let new_contiguous = [&contiguous; 5].into_iter().flatten().copied().collect();

    (new_map, new_contiguous)
}

fn recursive_solve(row: Row, memory: &mut HashMap<Row, u128>) -> u128 {
    if let Some(value) = memory.get(&row) {
        return *value;
    }

    let (map, contiguous) = row.clone();

    if contiguous.iter().sum::<usize>() + contiguous.len() > map.len() + 1 {
        return add_and_return(0, row, memory);
    }

    match (contiguous.is_empty(), map.contains('#'), map.is_empty()) {
        (true, false, _) => {
            return add_and_return(1, row, memory);
        }
        (true, true, _) | (false, _, true) => {
            return add_and_return(0, row, memory);
        }
        _ => (),
    }

    let start_pattern_regex_pattern = format!("^[\\?\\#]{{{}}}(?:[^\\#]|$)", contiguous[0]);
    let start_pattern_regex = regex::Regex::new(&start_pattern_regex_pattern).unwrap();

    let mut configs = 0;

    if let Some(start_match) = start_pattern_regex.find(&map) {
        let chars_to_skip = start_match.as_str().len();

        let start_pattern_map = remove_start_stops(map[chars_to_skip..].to_string());
        let start_pattern_contiguous = contiguous[1..].to_vec();

        configs += recursive_solve((start_pattern_map, start_pattern_contiguous), memory);
    }

    if map.starts_with('#') {
        return add_and_return(configs, row, memory);
    }

    let ans = configs
        + recursive_solve(
            (remove_start_stops(map[1..].to_string()), contiguous),
            memory,
        );
    add_and_return(ans, row, memory)
}

fn add_and_return(value: u128, input: Row, memory: &mut HashMap<Row, u128>) -> u128 {
    memory.insert(input, value);
    value
}

type Map = String;
type Contiguous = Vec<usize>;
type Row = (Map, Contiguous);
fn parse_file(file: &str, unwrap: bool) -> Vec<Row> {
    let contents = std::fs::read_to_string(file).unwrap();

    contents.lines().map(|line| parse_line(line, unwrap)).collect()
}

fn parse_line(line: &str, unfold: bool) -> Row {
    let mut split_line: std::str::Split<'_, char> = line.split(' ');

    let mut map = split_line.next().unwrap().to_string();

    let mut contiguous: Contiguous = split_line
        .next()
        .unwrap()
        .split(',')
        .map(|string| string.parse().unwrap())
        .collect();

    if unfold{
        let unfolded_row = unfold_row((map, contiguous));
        map = unfolded_row.0;
        contiguous = unfolded_row.1;
    }

    map = dedup_stops(map.trim_matches('.').to_string());

    (map, contiguous)
}

fn dedup_stops(line: String) -> String {
    let regex = regex::Regex::new(r"\.+").unwrap();
    regex.replace_all(&line, ".").to_string()
}

fn remove_start_stops(line: String) -> String {
    let regex = regex::Regex::new(r"^\.*").unwrap();
    regex.replace(&line, "").to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let row = parse_line("..??..??...?##.. 1,1,3", false);

        let expected_map = "??.??.?##";
        let expected_contiguous = vec![1, 1, 3];

        assert_eq!(row.0, expected_map);
        assert_eq!(row.1, expected_contiguous);
    }

    #[test]
    fn test_recursive_solve() {
        let inputs = [
            ("#.#.### 1,1,3", 1),
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];

        inputs.into_iter().for_each(|(input, expected)| {
            let row = parse_line(input, false);
            let actual = recurse_with_memory(vec![row]);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_number_possible_combinations() {
        let actual = number_possible_combinations("resources/2023/day12/test_input");
        assert_eq!(actual, 21);
    }

    #[test]
    fn test_unfold_recursive_solve() {
        let inputs = [
            // ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384),
            // ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            // ("????.#...#... 4,1,1", 16),
            // ("????.######..#####. 1,6,5", 2500),
            // ("?###???????? 3,2,1", 506250),
        ];

        inputs.into_iter().for_each(|(input, expected)| {
            let row = parse_line(input, true);
            let actual = recurse_with_memory(vec![row]);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_number_unfolded_possible_combinations() {
        let actual = number_unfolded_possible_combinations("resources/2023/day12/test_input");
        assert_eq!(actual, 525152);
    }
}
