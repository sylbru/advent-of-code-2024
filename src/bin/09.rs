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
    let mut blocks: Vec<Option<u8>> = Vec::new();
    let mut next_block_type = BlockType::File;
    let mut file_id = 0u8;

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

    None

    // track i_free index next free block
    // track i_to_move index next block to move
    // while i_free < i_to_move
}

// fn parse(input: &str) ->

// fn run_one() -> u32 {
//     42
// }

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
