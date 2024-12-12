use std::collections::HashMap;

pub mod task1 {
    pub fn ans() -> u128 {
        super::sum_region_prices("resources/2024/day12/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

fn sum_region_prices(file: &str) -> u128 {
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
    loop {
        let ((start_x, start_y), char) = match map.iter().next() {
            Some((pos, char)) => (*pos, *char),
            None => break,
        };

        let mut frontier = vec![(start_x, start_y)];

        let mut area = 0;
        let mut perimeter = 0;

        while let Some((x, y)) = frontier.pop() {
            if !map.contains_key(&(x, y)) {
                continue;
            }

            map.remove(&(x, y));

            area += 1;

            let neighbours = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|(x, y)| {
                    x >= &0 && x < &(width as isize) && y >= &0 && y < &(height as isize)
                })
                .filter(|(x, y)| content[*y as usize][*x as usize] == char)
                .collect::<Vec<(isize, isize)>>();

            perimeter += 4 - neighbours.len() as u128;

            let unvisited_neighbours = neighbours
                .into_iter()
                .filter(|(x, y)| map.contains_key(&(*x, *y)))
                .collect::<Vec<(isize, isize)>>();

            frontier.extend(unvisited_neighbours);
        }

        let price = area * perimeter;

        total_price += price;
    }

    total_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_region_prices() {
        assert_eq!(
            sum_region_prices("resources/2024/day12/test_input_1.txt"),
            140
        );

        assert_eq!(
            sum_region_prices("resources/2024/day12/test_input_2.txt"),
            1930
        );
    }
}
