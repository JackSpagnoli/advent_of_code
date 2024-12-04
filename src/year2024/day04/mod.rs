use std::{fs, rc::Rc};

pub mod task1 {
    use super::count_xmas;

    pub fn ans() -> u128 {
        count_xmas("resources/2024/day04/input.txt")
    }
}

pub mod task2 {
    use super::count_x_mas;

    pub fn ans() -> u128 {
        count_x_mas("resources/2024/day04/input.txt")
    }
}

fn count_xmas(file: &str) -> u128 {
    let map = fs::read_to_string(file)
        .expect("Error reading file")
        .lines()
        .map(|line| Rc::from(line))
        .collect::<Vec<Rc<str>>>();

    let map_rows = map.len();
    let map_cols = map[0].len();

    let row_indicies: Vec<[(usize, usize); 4]> = (0..map_rows)
        .flat_map(|y| {
            (0..map_cols - 3).map(move |start_x| {
                [
                    (y, start_x),
                    (y, start_x + 1),
                    (y, start_x + 2),
                    (y, start_x + 3),
                ]
            })
        })
        .collect();

    let col_indicies: Vec<[(usize, usize); 4]> = (0..map_rows - 3)
        .flat_map(|start_y| {
            (0..map_cols).map(move |x| {
                [
                    (start_y, x),
                    (start_y + 1, x),
                    (start_y + 2, x),
                    (start_y + 3, x),
                ]
            })
        })
        .collect();

    let r_diag_incides: Vec<[(usize, usize); 4]> = (0..map_rows - 3)
        .flat_map(|start_y| {
            (0..map_cols - 3).map(move |start_x| {
                [
                    (start_y, start_x),
                    (start_y + 1, start_x + 1),
                    (start_y + 2, start_x + 2),
                    (start_y + 3, start_x + 3),
                ]
            })
        })
        .collect();

    let l_diag_incides: Vec<[(usize, usize); 4]> = (0..map_rows - 3)
        .flat_map(|start_y| {
            (3..map_cols).map(move |start_x| {
                [
                    (start_y, start_x),
                    (start_y + 1, start_x - 1),
                    (start_y + 2, start_x - 2),
                    (start_y + 3, start_x - 3),
                ]
            })
        })
        .collect();

    row_indicies
        .into_iter()
        .chain(col_indicies)
        .chain(r_diag_incides)
        .chain(l_diag_incides)
        .map(|indices| {
            indices
                .iter()
                .map(|index| get_char(&map, *index))
                .collect::<String>()
        })
        .filter(|word| word == "XMAS" || word == "SAMX")
        .count() as u128
}

fn count_x_mas(file: &str) -> u128 {
    let map = fs::read_to_string(file)
        .expect("Error reading file")
        .lines()
        .map(|line| Rc::from(line))
        .collect::<Vec<Rc<str>>>();

    let map_rows = map.len();
    let map_cols = map[0].len();

    (0..map_rows - 2)
        .flat_map(|y| {
            (0..map_cols - 2).map(move |x| {
                [
                    (y + 1, x + 1),
                    (y, x),
                    (y + 2, x + 2),
                    (y + 2, x),
                    (y, x + 2),
                ]
            })
        })
        .map(|indicies| {
            let center = get_char(&map, indicies[0]);
            let top_left = get_char(&map, indicies[1]);
            let bottom_right = get_char(&map, indicies[2]);
            let bottom_left = get_char(&map, indicies[3]);
            let top_right = get_char(&map, indicies[4]);

            (center, top_left, bottom_right, bottom_left, top_right)
        })
        .filter(|(center, top_left, bottom_right, bottom_left, top_right)| {
            if center != &'A' {
                return false;
            }

            match (top_left, bottom_right) {
                ('M', 'S') | ('S', 'M') => (),
                _ => return false,
            };

            match (top_right, bottom_left) {
                ('M', 'S') | ('S', 'M') => (),
                _ => return false,
            };

            true
        })
        .count() as u128
}

fn get_char(map: &Vec<Rc<str>>, index: (usize, usize)) -> char {
    map[index.0].chars().nth(index.1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas("resources/2024/day04/test.txt"), 18);
    }

    #[test]
    fn test_count_x_mas() {
        assert_eq!(count_x_mas("resources/2024/day04/test.txt"), 9);
    }
}
