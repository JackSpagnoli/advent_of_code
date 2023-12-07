use regex::Regex;
use std::{cmp::max, fs};

pub mod task1 {
    use super::sum_possible_games;

    pub fn ans() -> u128 {
        sum_possible_games("resources/2023/day02/input")
    }
}

pub mod task2 {
    use super::sum_draw_powers;

    pub fn ans() -> u128 {
        sum_draw_powers("resources/2023/day02/input")
    }
}

fn sum_possible_games(file: &str) -> u128 {
    let file = fs::read_to_string(file).expect("Could not read file");
    file.lines()
        .map(parse_line)
        .filter(possible_games)
        .map(|game| game.id)
        .sum()
}

fn sum_draw_powers(file: &str) -> u128 {
    let file = fs::read_to_string(file).expect("Could not read file");
    file.lines()
        .map(parse_line)
        .map(min_possible_cubes)
        .map(draw_power)
        .sum()
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Draw {
    red: u128,
    green: u128,
    blue: u128,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u128,
    draws: Vec<Draw>,
}

fn parse_line(line: &str) -> Game {
    let regex = Regex::new(r"Game (?P<game_id>\d+): (?P<game>.*)").unwrap();
    let captures = regex.captures(line).unwrap();
    let game_id = captures["game_id"].parse::<u128>().unwrap();

    let draws = captures["game"]
        .split("; ")
        .map(parse_draw)
        .collect::<Vec<Draw>>();

    Game { id: game_id, draws }
}

fn parse_draw(draw_string: &str) -> Draw {
    let mut draw = Draw {
        red: 0,
        green: 0,
        blue: 0,
    };

    let regex = Regex::new(r"(?P<value>\d+) (?P<colour>red|blue|green)").unwrap();
    draw_string.split(", ").for_each(|cube| {
        let captures = regex.captures(cube).unwrap();
        let colour = &captures["colour"];
        let value = captures["value"].parse::<u128>().unwrap();

        match colour {
            "red" => draw.red = value,
            "green" => draw.green = value,
            "blue" => draw.blue = value,
            _ => panic!("Unknown colour"),
        }
    });

    draw
}

fn possible_games(game: &Game) -> bool {
    let red = 12;
    let green = 13;
    let blue = 14;

    game.draws
        .iter()
        .all(|draw| draw.red <= red && draw.green <= green && draw.blue <= blue)
}

fn min_possible_cubes(game: Game) -> Draw {
    game.draws.iter().fold(
        Draw {
            red: 0,
            green: 0,
            blue: 0,
        },
        |acc, draw| Draw {
            red: max(acc.red, draw.red),
            green: max(acc.green, draw.green),
            blue: max(acc.blue, draw.blue),
        },
    )
}

fn draw_power(draw: Draw) -> u128 {
    draw.red * draw.green * draw.blue
}

#[cfg(test)]
mod test {
    use super::{sum_possible_games, Draw, Game};

    #[test]
    fn test_parse_draw() {
        let draws = vec![
            (
                "3 blue, 4 red",
                Draw {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
            ),
            (
                "1 red, 2 green",
                Draw {
                    red: 1,
                    green: 2,
                    blue: 0,
                },
            ),
            (
                "6 blue, 2 green",
                Draw {
                    red: 0,
                    green: 2,
                    blue: 6,
                },
            ),
            (
                "5 blue, 4 red, 13 green",
                Draw {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
            ),
        ];

        draws.iter().for_each(|(input, expected)| {
            let actual = super::parse_draw(input);
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn test_parse_line() {
        let lines = vec![
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                Game {
                    id: 1,
                    draws: vec![
                        Draw {
                            red: 4,
                            green: 0,
                            blue: 3,
                        },
                        Draw {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Draw {
                            red: 0,
                            green: 2,
                            blue: 0,
                        },
                    ],
                },
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                Game {
                    id: 3,
                    draws: vec![
                        Draw {
                            red: 20,
                            green: 8,
                            blue: 6,
                        },
                        Draw {
                            red: 4,
                            green: 13,
                            blue: 5,
                        },
                        Draw {
                            red: 1,
                            green: 5,
                            blue: 0,
                        },
                    ],
                },
            ),
        ];

        lines.iter().for_each(|(input, expected)| {
            let actual = super::parse_line(input);
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn test_sum_possible_games() {
        let file = "resources/2023/day02/test_input";
        let expected = 8;
        let actual = sum_possible_games(file);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_min_possible_cubes() {
        let game = Game {
            id: 1,
            draws: vec![
                Draw {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Draw {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Draw {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };

        let expected = Draw {
            red: 4,
            green: 2,
            blue: 6,
        };

        let actual = super::min_possible_cubes(game);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_draw_power() {
        let draws = [(
                Draw {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                0,
            ),
            (
                Draw {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                12,
            ),
            (
                Draw {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
                0,
            )];

        draws.iter().for_each(|(draw, expected)| {
            let actual = super::draw_power(*draw);
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn test_sum_draw_powers() {
        let file = "resources/2023/day02/test_input";
        let expected = 2286;
        let actual = super::sum_draw_powers(file);
        assert_eq!(actual, expected);
    }
}
