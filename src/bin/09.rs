#![allow(unused)]

advent_of_code::solution!(9);

fn parse(input: &str) -> Option<u32> {
    Some(42)
}

enum BlockType {
    Free,
    File,
}

pub fn part_one(input: &str) -> Option<u32> {
    // convert disk map into blocks
    let mut blocks: Vec<Option<usize>> = Vec::new();
    let mut next_block_type = BlockType::File;
    let mut file_id = 0usize;

    for c in input.chars() {
        let length = c.to_string().parse::<u8>().ok()?;

        for _i in 0..length {
            match next_block_type {
                BlockType::Free => blocks.push(None),
                BlockType::File => blocks.push(Some(file_id)),
            }
        }

        // alternate block types
        // if we were on a file block type, increment file id
        match next_block_type {
            BlockType::Free => next_block_type = BlockType::File,
            BlockType::File => {
                next_block_type = BlockType::Free;
                file_id += 1;
            }
        };
    }

    // println!("{:?}", blocks);

    let mut i_free = 0;
    let mut i_to_move = blocks.len() - 1;

    while i_free < i_to_move {
        while blocks[i_free].is_some() {
            i_free += 1;
        }

        while blocks[i_to_move].is_none() {
            i_to_move -= 1;
        }

        if i_free < i_to_move {
            blocks.swap(i_free, i_to_move);
        }
    }

    // println!("{:?}", blocks);
    Some(checksum(blocks))
}

fn checksum(blocks: Vec<Option<usize>>) -> u32 {
    let mut result = 0usize;

    for (i, block) in blocks.iter().enumerate() {
        if let Some(file_id) = block {
            // println!("{} {:?}", i, file_id);
            result += i * (*file_id);
        }
    }

    result as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
