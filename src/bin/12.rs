#![allow(unused)]

advent_of_code::solution!(12);

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: u8,
    y: u8,
}

type Region = HashSet<Position>;

pub fn part_one(input: &str) -> Option<u32> {
    let mut position_to_region: HashMap<Position, Option<&Region>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, plant) in line.chars().enumerate() {
            // if x,y not already in a region, start exploring
            let pos = Position {
                x: x as u8,
                y: y as u8,
            };
            if !position_to_region.contains_key(&pos) {
                println!("{:?}", pos);
            }
        }
    }

    None
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
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
