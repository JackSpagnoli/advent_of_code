use std::cmp::{max, min};

pub mod task1 {
    use super::lowest_location_number;

    pub fn ans() -> u128 {
        lowest_location_number("resources/2023/day05/input")
    }
}

pub mod task2 {
    use super::lowest_location_number_from_range;

    pub fn ans() -> u128 {
        lowest_location_number_from_range("resources/2023/day05/input")
    }
}

#[derive(PartialEq, Debug)]
struct Map {
    input_start: u128,
    output_start: u128,
    range: u128,
    input_range: SeedRange,
}

struct SeedRangeMap {
    mapped_input: Option<SeedRange>,
    mapped_output: Option<SeedRange>,
    unmapped_left: Option<SeedRange>,
    unmapped_right: Option<SeedRange>,
}

struct SeedRangesMap {
    mapped_input: Vec<SeedRange>,
    mapped_output: Vec<SeedRange>,
    unmapped: Vec<SeedRange>,
}

impl Map {
    fn map(&self, input: u128) -> Option<u128> {
        if !(self.input_start..self.input_start + self.range).contains(&input) {
            return None;
        }
        Some((input as i128 + self.diff()) as u128)
    }

    fn diff(&self) -> i128 {
        self.output_start as i128 - self.input_start as i128
    }

    fn map_range(&self, input: SeedRange) -> SeedRangeMap {
        let range_intersection = Intersection::from((input, self.input_range));
        match range_intersection.intersection_range {
            Some(intersection) => {
                let mapped_intersection = SeedRange {
                    start: (intersection.start as i128 + self.diff()) as u128,
                    end: (intersection.end as i128 + self.diff()) as u128,
                };
                SeedRangeMap {
                    mapped_output: Some(mapped_intersection),
                    mapped_input: Some(intersection),
                    unmapped_left: range_intersection.left,
                    unmapped_right: range_intersection.right,
                }
            }
            None => SeedRangeMap {
                mapped_input: None,
                mapped_output: None,
                unmapped_left: range_intersection.left,
                unmapped_right: range_intersection.right,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
struct StageMap {
    maps: Vec<Map>,
}

impl StageMap {
    fn map(&self, input: u128) -> u128 {
        self.maps
            .iter()
            .find_map(|range| range.map(input))
            .unwrap_or(input)
    }

    fn map_range(&self, input: SeedRange) -> SeedRanges {
        let mapped_ranges: Vec<SeedRangeMap> = self
            .maps
            .iter()
            .map(|map| {
                map.map_range(input)
            }).collect();

        let mut ranges_map : SeedRangesMap= mapped_ranges
            .into_iter()
            .fold(SeedRangesMap{
                mapped_input: vec![],
                mapped_output: vec![],
                unmapped: vec![],
            }, |mut acc_map, range_map| {
                if let Some(mapped_input) = range_map.mapped_input {
                    acc_map.mapped_input.push(mapped_input);
                }
                if let Some(mapped_output) = range_map.mapped_output {
                    acc_map.mapped_output.push(mapped_output);
                }
                if let Some(unmapped_left) = range_map.unmapped_left {
                    acc_map.unmapped.push(unmapped_left);
                }
                if let Some(unmapped_right) = range_map.unmapped_right {
                    acc_map.unmapped.push(unmapped_right);
                }
                acc_map
            });

        let unmapped_ranges = remove_ranges(ranges_map.unmapped, ranges_map.mapped_input);

        let ranges = ranges_map.mapped_output.into_iter().chain(unmapped_ranges).collect();

        SeedRanges { ranges }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct SeedRange {
    start: u128,
    end: u128,
}

#[derive(Debug, PartialEq)]
struct SeedRanges {
    ranges: Vec<SeedRange>,
}

struct Intersection {
    intersection_range: Option<SeedRange>,
    left: Option<SeedRange>,
    right: Option<SeedRange>,
}

impl From<(SeedRange, SeedRange)> for Intersection {
    fn from(value: (SeedRange, SeedRange)) -> Self {
        // a: input, b: map
        let (input, map) = value;

        if map.start > input.end {
            return Intersection {
                intersection_range: None,
                left: Some(input),
                right: None,
            };
        }

        if input.start > map.end {
            return Intersection {
                intersection_range: None,
                left: None,
                right: Some(input),
            };
        }

        let intersection = Some(SeedRange {
            start: max(input.start, map.start),
            end: min(input.end, map.end),
        });

        let mut left = None;
        let mut right = None;

        if input.start < map.start {
            left = Some(SeedRange {
                start: input.start,
                end: map.start - 1,
            });
        }

        if input.end > map.end {
            right = Some(SeedRange {
                start: map.end + 1,
                end: input.end,
            });
        }

        Intersection {
            intersection_range: intersection,
            left,
            right,
        }
    }
}

fn lowest_location_number(file: &str) -> u128 {
    let contents = std::fs::read_to_string(file).expect("Could not read file");

    let mut blocks = contents.split("\n\n");
    let seeds = parse_seeds(blocks.next().unwrap());

    let maps = blocks.map(parse_map);

    let map_seed = |seed| maps.clone().fold(seed, |seed, map| map.map(seed));

    seeds.into_iter().map(map_seed).min().unwrap()
}

fn lowest_location_number_from_range(file: &str) -> u128 {
    let contents = std::fs::read_to_string(file).expect("Could not read file");

    let mut blocks = contents.split("\n\n");
    let mut seed_ranges: Vec<SeedRange> = parse_seeds(blocks.next().unwrap())
        .windows(2)
        .step_by(2)
        .map(|w| {
            let start = w[0];
            let end = start + w[1] - 1;
            SeedRange { start, end }
        })
        .collect();

    let maps = blocks.map(parse_map);

    seed_ranges = maps.fold(seed_ranges, |seed_ranges, map| {
        let ranges = map_ranges(seed_ranges, &map);
        reduce_ranges(SeedRanges { ranges }).ranges
    });

    seed_ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn map_ranges(ranges: Vec<SeedRange>, map: &StageMap) -> Vec<SeedRange> {
    println!("Using map: {:?}", map);
    ranges
        .into_iter()
        .flat_map(|range| {
            println!("Mapping range {:?}", range);
            let r = map.map_range(range).ranges;
            println!("To range {:?}\n", r);
            r
        })
        .collect()
}

fn parse_map(input: &str) -> StageMap {
    let ranges = input
        .split('\n')
        .skip(1)
        .map(|range| {
            let mut nums = range.split(' ');
            let output_start = nums.next().unwrap().parse::<u128>().unwrap();
            let input_start = nums.next().unwrap().parse::<u128>().unwrap();
            let range = nums.next().unwrap().parse::<u128>().unwrap();
            let input_range = SeedRange {
                start: input_start,
                end: input_start + range - 1,
            };
            Map {
                input_start,
                output_start,
                range,
                input_range,
            }
        })
        .collect();
    StageMap { maps: ranges }
}

fn parse_seeds(input: &str) -> Vec<u128> {
    input
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<_>>()
}

fn reduce_ranges(ranges: SeedRanges) -> SeedRanges {
    let mut reduced_ranges = vec![];
    let mut ranges = ranges.ranges;
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut current_range = ranges[0];
    for range in ranges.into_iter().skip(1) {
        if range.start <= current_range.end + 1 {
            current_range.end = max(range.end, current_range.end);
        } else {
            reduced_ranges.push(current_range);
            current_range = range;
        }
    }
    reduced_ranges.push(current_range);
    SeedRanges {
        ranges: reduced_ranges,
    }
}

fn remove_ranges(ranges: Vec<SeedRange>, to_remove: Vec<SeedRange>) -> Vec<SeedRange> {
    let mut ranges = ranges;
    for range_to_remove in to_remove{
        ranges = ranges.into_iter().flat_map(|range| {
            let intersection = Intersection::from((range, range_to_remove));
            match intersection.intersection_range {
                Some(_) => {
                    let mut new_ranges = vec![];
                    if let Some(left) = intersection.left {
                        new_ranges.push(left);
                    }
                    if let Some(right) = intersection.right {
                        new_ranges.push(right);
                    }
                    new_ranges
                }
                None => vec![range],
            }
        }).collect();
    }
    
    ranges
}

#[cfg(test)]
mod test {
    use crate::year2023::day05::*;

    #[test]
    fn test_parse_map() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48";
        let map = parse_map(input);
        let expected_map = StageMap {
            maps: vec![
                Map {
                    input_start: 98,
                    output_start: 50,
                    range: 2,
                    input_range: SeedRange { start: 98, end: 99 },
                },
                Map {
                    input_start: 50,
                    output_start: 52,
                    range: 48,
                    input_range: SeedRange { start: 50, end: 97 },
                },
            ],
        };

        assert_eq!(map.maps, expected_map.maps);
    }

    #[test]
    fn test_apply_map() {
        let map = parse_map("seed-to-soil map:\n50 98 2\n52 50 48");

        assert_eq!(map.map(0), 0);
        assert_eq!(map.map(53), 55);
        assert_eq!(map.map(99), 51);
    }

    #[test]
    fn test_lowest_location_number() {
        assert_eq!(
            lowest_location_number("resources/2023/day05/test_input"),
            35
        );
    }

    #[test]
    fn test_map_ranges() {
        let map = parse_map("seed-to-soil map:\n50 98 2\n52 50 48");
        let input = vec![
            SeedRange { start: 79, end: 92 },
            SeedRange { start: 55, end: 67 },
        ];

        let expected = vec![
            SeedRange { start: 55, end: 69 },
            SeedRange { start: 79, end: 94 },
        ];

        let ans = map_ranges(input, &map);
        let ans = reduce_ranges(SeedRanges {
            ranges: ans.clone(),
        })
        .ranges;

        assert_eq!(ans, expected);
    }

    #[test]
    fn test_map_ranges2() {
        let map = parse_map("soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15");
        map.maps.iter().for_each(|m| {
            println!(
                "[{},{}] -> [{},{}]",
                m.input_start,
                m.input_start + m.range - 1,
                m.output_start,
                m.output_start + m.range - 1
            )
        });
        let input = vec![
            SeedRange { start: 55, end: 69 },
            SeedRange { start: 79, end: 94 },
        ];

        let expected = vec![
            SeedRange { start: 55, end: 69 },
            SeedRange { start: 79, end: 94 },
        ];

        let ans = map_ranges(input, &map);
        let ans = reduce_ranges(SeedRanges {
            ranges: ans.clone(),
        })
        .ranges;

        assert_eq!(ans, expected);
    }

    #[test]
    fn test_map_ranges3() {
        let map = parse_map("fertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4");
        map.maps.iter().for_each(|m| {
            println!(
                "[{},{}] -> [{},{}]",
                m.input_start,
                m.input_start + m.range - 1,
                m.output_start,
                m.output_start + m.range - 1
            )
        });
        let input = vec![
            SeedRange { start: 55, end: 69 },
            SeedRange { start: 79, end: 94 },
        ];

        let expected = vec![
            SeedRange { start: 51, end: 69 },
            SeedRange { start: 79, end: 94 },
        ];

        let ans = map_ranges(input, &map);
        let ans = reduce_ranges(SeedRanges {
            ranges: ans.clone(),
        })
        .ranges;

        assert_eq!(ans, expected);
    }

    #[test]
    fn test_map_ranges4() {
        let map = parse_map("water-to-light map:\n88 18 7\n18 25 70");
        map.maps.iter().for_each(|m| {
            println!(
                "[{},{}] -> [{},{}]",
                m.input_start,
                m.input_start + m.range - 1,
                m.output_start,
                m.output_start + m.range - 1
            )
        });
        let input = vec![
            SeedRange { start: 51, end: 69 },
            SeedRange { start: 79, end: 94 },
        ];

        let expected = vec![
            SeedRange { start: 44, end: 69 },
            SeedRange { start: 72, end: 94 },
        ];

        let ans = map_ranges(input, &map);
        let ans = reduce_ranges(SeedRanges {
            ranges: ans.clone(),
        })
        .ranges;

        assert_eq!(ans, expected);
    }

    #[test]
    fn test_map_ranges5() {
        let map = parse_map("light-to-temperature map:\n45 77 23\n81 45 19\n68 64 13");
        map.maps.iter().for_each(|m| {
            println!(
                "[{},{}] -> [{},{}]",
                m.input_start,
                m.input_start + m.range - 1,
                m.output_start,
                m.output_start + m.range - 1
            )
        });
        let input = vec![
            SeedRange { start: 44, end: 69 },
            SeedRange { start: 72, end: 94 },
        ];

        let expected = vec![SeedRange { start: 44, end: 99 }];

        let ans = map_ranges(input, &map);
        let ans = reduce_ranges(SeedRanges {
            ranges: ans.clone(),
        })
        .ranges;

        assert_eq!(ans, expected);
    }

    #[test]
    fn test_map_ranges6() {
        let map = parse_map("temperature-to-humidity map:\n0 69 1\n1 0 69");
        map.maps.iter().for_each(|m| {
            println!(
                "[{},{}] -> [{},{}]",
                m.input_start,
                m.input_start + m.range - 1,
                m.output_start,
                m.output_start + m.range - 1
            )
        });
        let input = vec![SeedRange { start: 44, end: 99 }];

        let expected = vec![
            SeedRange { start: 0, end: 0 },
            SeedRange { start: 45, end: 99 },
        ];

        let ans = map_ranges(input, &map);
        let ans = reduce_ranges(SeedRanges {
            ranges: ans.clone(),
        })
        .ranges;

        assert_eq!(ans, expected);
    }
    #[test]
    fn test_lowest_location_number_from_range() {
        assert_eq!(
            lowest_location_number_from_range("resources/2023/day05/test_input"),
            46
        );
    }
}
