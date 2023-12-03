use std::fs;

use regex::{Match, Regex};

pub mod task1 {
    use super::sum_engine_part_numbers;

    pub fn ans() -> u128 {
        sum_engine_part_numbers("resources/day03/input")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Number {
    value: u128,
    len: usize,
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

#[derive(Default, PartialEq, Debug)]
struct Map {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Map {
    fn filter_part_numbers(mut self) -> Self {
        self.numbers = self
            .numbers
            .into_iter()
            .filter(|number| {
                let x_range = number.x as isize-1..=(number.x + number.len+1) as isize;
                let y_range = number.y as isize-1..=(number.y + 1) as isize;

                self.symbols.iter().any(|symbol| {
                    x_range.contains(&(symbol.x as isize)) && y_range.contains(&(symbol.y as isize))
                })
            })
            .collect();

        self
    }

    fn sum_part_numbers(&self) -> u128 {
        self.numbers.iter().map(|number| number.value).sum()
    }
}

fn sum_engine_part_numbers(file: &str) -> u128 {
    let file = fs::read_to_string(file).expect("Could not read file");

    parse_file(&file).filter_part_numbers().sum_part_numbers()
}

fn parse_file(file: &str) -> Map {
    file.lines()
        .enumerate()
        .fold(Map::default(), |map, (y, line)| parse_line(map, y, line))
}

fn parse_line(map: Map, y: usize, line: &str) -> Map {
    let regex = Regex::new(r"(?P<number>\d+)|(?P<symbol>[^\d|.|\s])").unwrap();
    regex.captures_iter(line).fold(map, |mut map, capture| {
        if let Some(number) = capture.name("number") {
            map.numbers.push(parse_number(number, y));
        } else if let Some(symbol) = capture.name("symbol") {
            map.symbols.push(parse_symbol(symbol, y));
        }
        map
    })
}

fn parse_number(number: Match, y: usize) -> Number {
    Number {
        value: number.as_str().parse::<u128>().unwrap(),
        len: number.as_str().len(),
        x: number.start(),
        y,
    }
}

fn parse_symbol(symbol: Match, y: usize) -> Symbol {
    Symbol {
        x: symbol.start(),
        y,
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "467..114..";
        let map = parse_line(Map::default(), 0, line);

        assert_eq!(map.numbers.len(), 2);
        assert_eq!(map.numbers[0].value, 467);
        assert_eq!(map.numbers[0].len, 3);
        assert_eq!(map.numbers[0].x, 0);
        assert_eq!(map.numbers[0].y, 0);

        assert_eq!(map.numbers[1].value, 114);
        assert_eq!(map.numbers[1].len, 3);
        assert_eq!(map.numbers[1].x, 5);
        assert_eq!(map.numbers[1].y, 0);
    }

    #[test]
    fn test_parse_line_symbols() {
        let line = "617*......";
        let map = parse_line(Map::default(), 0, line);

        assert_eq!(map.numbers.len(), 1);
        assert_eq!(map.numbers[0].value, 617);
        assert_eq!(map.numbers[0].len, 3);
        assert_eq!(map.numbers[0].x, 0);
        assert_eq!(map.numbers[0].y, 0);

        assert_eq!(map.symbols.len(), 1);
        assert_eq!(map.symbols[0].x, 3);
        assert_eq!(map.symbols[0].y, 0);
    }

    #[test]
    fn test_parse_lines() {
        let file = fs::read_to_string("resources/day03/test_input").expect("Could not read file");

        let map = parse_file(&file);

        let expected_numbers = vec![
            Number {
                value: 467,
                len: 3,
                x: 0,
                y: 0,
            },
            Number {
                value: 114,
                len: 3,
                x: 5,
                y: 0,
            },
            Number {
                value: 35,
                len: 2,
                x: 2,
                y: 2,
            },
            Number {
                value: 633,
                len: 3,
                x: 6,
                y: 2,
            },
            Number {
                value: 617,
                len: 3,
                x: 0,
                y: 4,
            },
            Number {
                value: 58,
                len: 2,
                x: 7,
                y: 5,
            },
            Number {
                value: 592,
                len: 3,
                x: 2,
                y: 6,
            },
            Number {
                value: 755,
                len: 3,
                x: 6,
                y: 7,
            },
            Number {
                value: 664,
                len: 3,
                x: 1,
                y: 9,
            },
            Number {
                value: 598,
                len: 3,
                x: 5,
                y: 9,
            },
        ];

        let expected_symbols = vec![
            Symbol { x: 3, y: 1 },
            Symbol { x: 6, y: 3 },
            Symbol { x: 3, y: 4 },
            Symbol { x: 5, y: 5 },
            Symbol { x: 3, y: 8 },
            Symbol { x: 5, y: 8 },
        ];

        let expected_map = Map {
            numbers: expected_numbers,
            symbols: expected_symbols,
        };

        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_filter_engine_parts() {
        let file = fs::read_to_string("resources/day03/test_input").expect("Could not read file");

        let map = parse_file(&file).filter_part_numbers();

        let expected_numbers = vec![
            Number {
                value: 467,
                len: 3,
                x: 0,
                y: 0,
            },
            Number {
                value: 35,
                len: 2,
                x: 2,
                y: 2,
            },
            Number {
                value: 633,
                len: 3,
                x: 6,
                y: 2,
            },
            Number {
                value: 617,
                len: 3,
                x: 0,
                y: 4,
            },
            Number {
                value: 592,
                len: 3,
                x: 2,
                y: 6,
            },
            Number {
                value: 755,
                len: 3,
                x: 6,
                y: 7,
            },
            Number {
                value: 664,
                len: 3,
                x: 1,
                y: 9,
            },
            Number {
                value: 598,
                len: 3,
                x: 5,
                y: 9,
            },
        ];

        assert_eq!(map.numbers, expected_numbers);
    }

    #[test]
    fn test_sum_part_numbers() {
        let actual = sum_engine_part_numbers("resources/day03/test_input");

        let expected = 4361;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_part_numbers2() {
        let actual = sum_engine_part_numbers("resources/day03/test_input2");

        let expected = 925;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_part_numbers3() {
        let actual = sum_engine_part_numbers("resources/day03/test_input3");

        let expected = 156;

        assert_eq!(actual, expected);
    }
}
