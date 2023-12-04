use std::fs;

use regex::{Match, Regex};

pub mod task1 {
    use super::sum_engine_part_numbers;

    pub fn ans() -> u128 {
        sum_engine_part_numbers("resources/day03/input")
    }
}

pub mod task2 {
    use super::sum_engine_gear_ratios;

    pub fn ans() -> u128 {
        sum_engine_gear_ratios("resources/day03/input")
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Number {
    value: u128,
    len: usize,
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum IsGear {
    Gear(usize),
    NotGear,
    PossibleGear,
}

#[derive(PartialEq, Debug)]
struct Symbol {
    x: usize,
    y: usize,
    is_gear: IsGear,
}

#[derive(Default, PartialEq, Debug)]
struct Map {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Map {
    fn filter_part_numbers(mut self) -> Self {
        self.numbers.retain(|number| {
            let (x_range, y_range) = get_number_range(number);

            self.symbols.iter().any(|symbol| {
                x_range.contains(&(symbol.x as isize)) && y_range.contains(&(symbol.y as isize))
            })
        });

        self
    }

    fn sum_part_numbers(&self) -> u128 {
        self.numbers.iter().map(|number| number.value).sum()
    }

    fn calculate_gear_ratios(mut self) -> Self {
        let (possible_gears, not_gears): (Vec<_>, Vec<_>) =
            self.symbols.into_iter().partition(possible_gear);

        let gears = possible_gears
            .into_iter()
            .map(|gear| get_gear_ratio(gear, &self.numbers));

        self.symbols = gears.chain(not_gears).collect();

        self
    }

    fn sum_gear_ratios(self) -> u128 {
        self.symbols
            .iter()
            .map(|symbol| match symbol.is_gear {
                IsGear::Gear(ratio) => ratio as u128,
                IsGear::NotGear => 0,
                IsGear::PossibleGear => panic!("Possible gear found"),
            })
            .sum()
    }
}

fn get_number_range(
    number: &Number,
) -> (
    std::ops::RangeInclusive<isize>,
    std::ops::RangeInclusive<isize>,
) {
    let min_x = number.x as isize - 1;
    let max_x = (number.x + number.len) as isize;
    let min_y = number.y as isize - 1;
    let max_y = (number.y + 1) as isize;

    let x_range = min_x..=max_x;
    let y_range = min_y..=max_y;

    (x_range, y_range)
}

fn possible_gear(symbol: &Symbol) -> bool {
    symbol.is_gear == IsGear::PossibleGear
}

fn get_gear_ratio(gear: Symbol, numbers: &[Number]) -> Symbol {
    let touching_numbers = numbers
        .iter()
        .filter(|number| {
            let (x_range, y_range) = get_number_range(number);

            x_range.contains(&(gear.x as isize)) && y_range.contains(&(gear.y as isize))
        })
        .collect::<Vec<_>>();

    if touching_numbers.len() != 2 {
        Symbol {
            x: gear.x,
            y: gear.y,
            is_gear: IsGear::NotGear,
        }
    } else {
        Symbol {
            x: gear.x,
            y: gear.y,
            is_gear: IsGear::Gear(
                touching_numbers[0].value as usize * touching_numbers[1].value as usize,
            ),
        }
    }
}

fn sum_engine_part_numbers(file: &str) -> u128 {
    let file = fs::read_to_string(file).expect("Could not read file");

    parse_file(&file).filter_part_numbers().sum_part_numbers()
}

fn sum_engine_gear_ratios(file: &str) -> u128 {
    let file = fs::read_to_string(file).expect("Could not read file");

    parse_file(&file).calculate_gear_ratios().sum_gear_ratios()
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
    let is_gear = match symbol.as_str() {
        "*" => IsGear::PossibleGear,
        _ => IsGear::NotGear,
    };
    Symbol {
        x: symbol.start(),
        y,
        is_gear,
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
            Symbol {
                x: 3,
                y: 1,
                is_gear: IsGear::PossibleGear,
            },
            Symbol {
                x: 6,
                y: 3,
                is_gear: IsGear::NotGear,
            },
            Symbol {
                x: 3,
                y: 4,
                is_gear: IsGear::PossibleGear,
            },
            Symbol {
                x: 5,
                y: 5,
                is_gear: IsGear::NotGear,
            },
            Symbol {
                x: 3,
                y: 8,
                is_gear: IsGear::NotGear,
            },
            Symbol {
                x: 5,
                y: 8,
                is_gear: IsGear::PossibleGear,
            },
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

    #[test]
    fn test_sum_gear_ratios() {
        let actual = sum_engine_gear_ratios("resources/day03/test_input");

        let expected = 467835;

        assert_eq!(actual, expected);
    }
}
