#![allow(unused)]
advent_of_code::solution!(8);

// parse into list of antennas positions, + dimensions
// (create bool matrix of those dimensions)
// all pairs of antennas of the same frequency
// for each pair, create two antinodes
// ignore out of bounds antinodes
// count unique antinodes (or count true values in bool matrix)

struct Position {
    x: i8,
    y: i8,
}

struct Map {
    dimension: u8,
    antennas: Vec<Position>,
}

// type Antinode = Position

fn parse(input: &str) -> Option<Map> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    Some(Map {
        dimension: matrix.len() as u8,
        antennas: vec![],
    })
}

fn run_one(map: Map) -> u32 {
    42
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(run_one)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
