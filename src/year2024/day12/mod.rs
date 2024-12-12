use std::collections::{HashMap, HashSet};

pub mod task1 {
    pub fn ans() -> u128 {
        super::sum_region_prices("resources/2024/day12/input.txt").0
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        super::sum_region_prices("resources/2024/day12/input.txt").1
    }
}

#[derive(Debug)]
enum Edge {
    Left,
    Right,
}

fn sum_region_prices(file: &str) -> (u128, u128) {
    let content = std::fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = content.len();
    let width = content[0].len();

    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    content.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &cell)| {
            map.insert((x as isize, y as isize), cell);
        });
    });

    let mut total_price = 0;
    let mut total_bulk_price = 0;
    loop {
        let ((start_x, start_y), char) = match map.iter().next() {
            Some((pos, char)) => (*pos, *char),
            None => break,
        };

        let mut frontier = vec![(start_x, start_y)];

        let mut perimeter = 0;
        let mut tiles: HashSet<(isize, isize)> = HashSet::new();

        while let Some((x, y)) = frontier.pop() {
            if !map.contains_key(&(x, y)) {
                continue;
            }

            map.remove(&(x, y));
            tiles.insert((x, y));

            let neighbours = vec![(x + 1, y), (x, y - 1), (x - 1, y), (x, y + 1)]
                .into_iter()
                .filter(|(x, y)| {
                    x >= &0 && x < &(width as isize) && y >= &0 && y < &(height as isize)
                })
                .filter(|(x, y)| content[*y as usize][*x as usize] == char)
                .collect::<Vec<(isize, isize)>>();

            perimeter += 4 - neighbours.len();

            let unvisited_neighbours = neighbours
                .into_iter()
                .filter(|(x, y)| map.contains_key(&(*x, *y)))
                .collect::<Vec<(isize, isize)>>();

            frontier.extend(unvisited_neighbours);
        }

        let area = tiles.len() as u128;
        let price = area * perimeter as u128;
        total_price += price;

        // Count number of sides by passing down and right and counting edge tiles
        let (minx, miny, maxx, maxy) = tiles.iter().fold(
            (
                std::isize::MAX,
                std::isize::MAX,
                std::isize::MIN,
                std::isize::MIN,
            ),
            |(minx, miny, maxx, maxy), (x, y)| {
                (minx.min(*x), miny.min(*y), maxx.max(*x), maxy.max(*y))
            },
        );

        let mut sides = 0;

        // rows
        for y in (miny - 1)..=maxy {
            let mut on_edge: Option<Edge> = None;
            for x in (minx - 1)..=maxx {
                let left_on_shape = tiles.contains(&(x, y));
                let right_on_shape = tiles.contains(&(x, y + 1));

                if left_on_shape && right_on_shape {
                    on_edge = None;
                    continue;
                }

                if left_on_shape && !right_on_shape {
                    if let Some(Edge::Left) = on_edge {
                        continue;
                    }
                    on_edge = Some(Edge::Left);
                    sides += 1;
                }

                if right_on_shape && !left_on_shape {
                    if let Some(Edge::Right) = on_edge {
                        continue;
                    }
                    on_edge = Some(Edge::Right);
                    sides += 1;
                }

                if !left_on_shape && !right_on_shape {
                    on_edge = None;
                }
            }
        }

        // columns
        for x in (minx - 1)..=maxx {
            let mut on_edge: Option<Edge> = None;
            for y in (miny - 1)..=maxy {
                let left_on_shape = tiles.contains(&(x, y));
                let right_on_shape = tiles.contains(&(x + 1, y));

                if left_on_shape == right_on_shape {
                    on_edge = None;
                    continue;
                }

                if left_on_shape && !right_on_shape {
                    if let Some(Edge::Left) = on_edge {
                        continue;
                    }
                    on_edge = Some(Edge::Left);
                    sides += 1;
                }

                if right_on_shape && !left_on_shape {
                    if let Some(Edge::Right) = on_edge {
                        continue;
                    }
                    on_edge = Some(Edge::Right);
                    sides += 1;
                }
            }
        }

        let bulk_price = area * (sides) as u128;
        total_bulk_price += bulk_price;
    }

    (total_price, total_bulk_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_region_prices() {
        assert_eq!(
            sum_region_prices("resources/2024/day12/test_input_1.txt"),
            (140, 80)
        );

        assert_eq!(
            sum_region_prices("resources/2024/day12/test_input_2.txt"),
            (1930, 1206)
        );

        assert_eq!(
            sum_region_prices("resources/2024/day12/test_input_3.txt").1,
            236
        );

        assert_eq!(
            sum_region_prices("resources/2024/day12/test_input_4.txt").1,
            368
        );

        assert_eq!(
            sum_region_prices("resources/2024/day12/test_input_5.txt").1,
            4
        );
    }
}
