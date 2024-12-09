pub mod task1 {
    use super::sorted_checksum;

    pub fn ans() -> u128 {
        sorted_checksum("resources/2024/day09/input.txt")
    }
}

pub mod task2 {
    use super::defrag_checksum;

    pub fn ans() -> u128 {
        defrag_checksum("resources/2024/day09/input.txt")
    }
}

fn sorted_checksum(file: &str) -> u128 {
    let content: &mut [u8] = &mut std::fs::read_to_string(file)
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

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
        let left = content[left_pointer];
        let right = content[right_pointer];

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
            content[left_pointer] = new_left;
        } else {
            // Remaining case where left pointer is empty and right pointer is data
            content[right_pointer] -= 1;
            content[left_pointer] -= 1;

            checksum += file_id(right_pointer) * left_block_pos;
            left_block_pos += 1;
        }
    }

    checksum
}

#[derive(Debug)]
enum Block {
    Empty(u8),
    Data(u8, usize),
}

impl Block {
    fn size(&self) -> u8 {
        match self {
            Block::Empty(size) => *size,
            Block::Data(size, _) => *size,
        }
    }

    fn id(&self) -> Option<usize> {
        match self {
            Block::Empty(_) => None,
            Block::Data(_, id) => Some(*id),
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Block::Empty(_))
    }
}

fn defrag_checksum(file: &str) -> u128 {
    let content: &mut [u8] = &mut std::fs::read_to_string(file)
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    let mut blocks = content
        .into_iter()
        .enumerate()
        .fold(
            (Vec::new(), true),
            |(mut blocks, is_file), (index, block_size)| {
                if is_file {
                    blocks.push(Block::Data(*block_size, index / 2));
                } else {
                    blocks.push(Block::Empty(*block_size));
                }

                (blocks, !is_file)
            },
        )
        .0;

    let length = blocks.len();

    // Defrag
    let mut file_block_index = length;
    while file_block_index > 0 {
        file_block_index -= 1;

        if blocks[file_block_index].is_empty() {
            continue;
        }

        let file_block = &blocks[file_block_index];
        let file_size = file_block.size();
        let file_id = file_block.id().unwrap();

        let empty_block_index = (0..file_block_index)
            .filter(|index| blocks[*index].is_empty())
            .find(|index| {
                let block = &blocks[*index];
                let block_size = block.size();

                block_size >= file_size
            });

        if let Some(empty_block_index) = empty_block_index {
            let empty_block = &blocks[empty_block_index];
            let empty_block_size = empty_block.size();

            if empty_block_size == file_size {
                blocks[empty_block_index] = Block::Data(file_size, file_id);
                blocks[file_block_index] = Block::Empty(empty_block_size);
            } else {
                let remainder = empty_block_size - file_size;
                blocks[empty_block_index] = Block::Data(file_size, file_id);
                blocks[file_block_index] = Block::Empty(file_size);

                blocks.insert(empty_block_index + 1, Block::Empty(remainder));
                file_block_index += 1;
            }
        }
    }

    blocks
        .into_iter()
        .fold(
            (0u128, 0u128),
            |(mut checksum, mut index), block| match block {
                Block::Empty(size) => (checksum, index + size as u128),
                Block::Data(size, id) => {
                    for _ in 0..size {
                        checksum += index as u128 * id as u128;
                        index += 1;
                    }

                    (checksum, index)
                }
            },
        )
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_checksum() {
        assert_eq!(sorted_checksum("resources/2024/day09/test_input.txt"), 1928);
    }

    #[test]
    fn test_defrag_checksum() {
        assert_eq!(defrag_checksum("resources/2024/day09/test_input.txt"), 2858);
    }
}
