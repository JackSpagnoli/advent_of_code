pub mod task1 {
    use super::sorted_checksum;

    pub fn ans() -> u128 {
        sorted_checksum("resources/2024/day09/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

fn sorted_checksum(file: &str) -> u128 {
    let content: &mut [char] = &mut std::fs::read_to_string(file)
        .unwrap()
        .trim()
        .chars()
        .collect::<Vec<char>>();

    let length = content.len();

    let mut end_is_data = true;
    let mut start_is_data = true;

    let mut left_pointer = 0;
    let mut left_block_pos = 0;
    let mut right_pointer = if length % 2 != 0 {
        length - 1
    } else {
        length - 2
    };

    let file_id = |pointer: usize| (pointer / 2) as u128;

    let mut checksum: u128 = 0;

    while left_pointer <= right_pointer {
        let left = content[left_pointer].to_digit(10).unwrap() as u128;
        let right = content[right_pointer].to_digit(10).unwrap() as u128;

        if !end_is_data || right == 0 {
            // Skip empty fields on right, or files already moved
            right_pointer -= 1;
            end_is_data = !end_is_data;
        } else if left == 0 {
            // Skip fields already accounted for on left
            left_pointer += 1;
            start_is_data = !start_is_data;
        } else if start_is_data {
            // Add checksum for left field and move on
            checksum += left_block_pos * file_id(left_pointer);
            left_block_pos += 1;

            let new_left = left - 1;
            content[left_pointer] = new_left.to_string().chars().next().unwrap();
        } else {
            // Remaining case where left pointer is empty and right pointer is data
            let new_right = right - 1;
            content[right_pointer] = new_right.to_string().chars().next().unwrap();

            let new_left = left - 1;
            content[left_pointer] = new_left.to_string().chars().next().unwrap();

            checksum += file_id(right_pointer) * left_block_pos;
            left_block_pos += 1;
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_checksum() {
        assert_eq!(sorted_checksum("resources/2024/day09/test_input.txt"), 1928);
    }
}
