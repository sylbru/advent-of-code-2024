#![allow(unused)]

advent_of_code::solution!(25);

#[derive(Debug)]
enum Schema {
    Key([u8; 5]),
    Lock([u8; 5]),
}

fn parse(input: &str) -> Vec<Schema> {
    input.split("\n\n").map(parse_schema).collect()
}

fn parse_schema(input: &str) -> Schema {
    let mut heights: [u8; 5] = [0, 0, 0, 0, 0];

    for line in input.lines() {
        for (i, character) in line.chars().enumerate() {
            if character == '#' {
                heights[i] += 1;
            }
        }
    }

    let adjusted_heights = heights.map(|height| height - 1);

    match input.chars().nth(0) {
        Some('#') => Schema::Lock(adjusted_heights),
        _ => Schema::Key(adjusted_heights),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse(input);

    None
}

fn key_and_lock_fit(key: Schema, lock: Schema) -> bool {
    match (key, lock) {
        (Schema::Key(heights_key), Schema::Lock(heights_lock)) => heights_key
            .iter()
            .zip(heights_lock.iter())
            .all(|(height_key, height_lock)| height_key + height_lock <= 5),
        _ => false,
    }
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
