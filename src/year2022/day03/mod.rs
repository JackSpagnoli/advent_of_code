use std::fs;
use substring::Substring;

pub mod task1 {
    use super::sum_priorities;

    pub fn ans() -> u128 {
        sum_priorities("resources/2022/day03/input")
    }
}

pub mod task2 {
    use super::sum_group_priorities;

    pub fn ans() -> u128 {
        sum_group_priorities("resources/2022/day03/input")
    }
}

fn sum_priorities(file: &str) -> u128 {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let split_contents = contents.lines();

    let score = split_contents.fold(0u32, |n, x| {
        let mut left_occurances: [u32; 52] = [0; 52];
        let mut right_occurances: [u32; 52] = [0; 52];
        let mut left_substring = x.substring(0, x.len() / 2).chars();
        let mut right_substring = x.substring(x.len() / 2, x.len()).chars();

        while left_substring.size_hint().0 > 0 {
            let mut left_char = (left_substring.next().unwrap() as usize) - 64;
            if left_char > 26 {
                left_char -= 32;
            } else {
                left_char += 26;
            }
            let mut right_char = (right_substring.next().unwrap() as usize) - 64;
            if right_char > 26 {
                right_char -= 32;
            } else {
                right_char += 26;
            }

            left_occurances[left_char - 1] += 1;
            right_occurances[right_char - 1] += 1;
        }

        let mut score_change: usize = 0;

        for i in 0..52 {
            if left_occurances[i] > 0 && right_occurances[i] > 0 {
                score_change += i + 1;
            }
        }
        n + (score_change as u32)
    });
    score as u128
}

fn sum_group_priorities(file: &str) -> u128 {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let mut split_contents = contents.lines();

    let total_groups = split_contents.clone().count() / 3;

    let mut score: usize = 0;

    for _group in 0..total_groups {
        let mut lines_contents: [[u32; 52]; 3] = [[0; 52]; 3];
        let mut lines = vec![
            split_contents.next().unwrap().chars(),
            split_contents.next().unwrap().chars(),
            split_contents.next().unwrap().chars()
        ];

        for i in 0..3 {
            while lines[i].size_hint().0 > 0 {
                let mut character = (lines[i].next().unwrap() as usize) - 64;
                if character > 26 {
                    character -= 32;
                } else {
                    character += 26;
                }

                lines_contents[i][character - 1] += 1;
            }
        }

        for i in 0..52 {
            if lines_contents[0][i] > 0 && lines_contents[1][i] > 0 && lines_contents[2][i] > 0 {
                score += i + 1;
            }
        }
    }
    score as u128
}