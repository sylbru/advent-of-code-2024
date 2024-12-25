#![allow(unused)]

advent_of_code::solution!(25);

#[derive(Debug)]
enum Schema {
    Key([u8; 5]),
    Lock([u8; 5]),
}

fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    for schema in input.split("\n\n") {
        match parse_schema(schema) {
            Schema::Lock(heights) => locks.push(heights),
            Schema::Key(heights) => keys.push(heights),
        }
    }

    (locks, keys)
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
    let (locks, keys) = parse(input);
    let mut combinations = 0;

    for lock in locks {
        combinations += keys
            .iter()
            .filter(|&key| key_and_lock_fit(key, &lock))
            .count()
    }

    Some(combinations as u32)
}

fn key_and_lock_fit(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    key.iter()
        .zip(lock.iter())
        .all(|(height_key, height_lock)| height_key + height_lock <= 5)
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
