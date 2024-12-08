#![allow(unused)]

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, Hash, Eq)]
struct Position {
    x: i8,
    y: i8,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Map {
    dimension: u8,
    antennas: Vec<Antenna>,
}

type Frequency = char;

#[derive(Debug)]
struct Antenna {
    frequency: Frequency,
    position: Position,
}

fn parse(input: &str) -> Option<Map> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dimension = matrix.len() as u8;
    let mut antennas = Vec::new();

    for y in 0..dimension {
        for x in 0..dimension {
            if matrix[y as usize][x as usize] != '.' {
                antennas.push(Antenna {
                    frequency: matrix[y as usize][x as usize],
                    position: Position {
                        x: x as i8,
                        y: y as i8,
                    },
                });
            }
        }
    }

    Some(Map {
        dimension,
        antennas,
    })
}

fn run_one(map: Map) -> u32 {
    /*
    - all pairs of antennas of the same frequency
    - antinodes : Vec<Position> or HashSet<Position>
    - for each pair, create two antinodes
    - ignore out of bounds antinodes
    - count unique antinodes (or just set length)(or count true values in bool matrix)
    */
    // initialise a hashmap: char to Vec<Position>
    // go through all antennas to build the hashmap
    // for each entry, build pairs then build antinodes
    let mut antennas_positions_by_frequency: HashMap<char, Vec<Position>> = HashMap::new();

    for antenna in map.antennas.iter() {
        antennas_positions_by_frequency
            .entry(antenna.frequency)
            .and_modify(|positions| positions.push(antenna.position))
            .or_insert(vec![antenna.position]);
    }

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (frequency, antennas_positions) in &antennas_positions_by_frequency {
        for (a, b) in antennas_positions.iter().tuple_combinations::<(_, _)>() {
            let dx = (b.x - a.x);
            let dy = (b.y - a.y);

            let antinode_a = Position {
                x: a.x - dx,
                y: a.y - dy,
            };
            if is_in_bounds(&antinode_a, map.dimension) {
                antinodes.insert(antinode_a.clone());
            }

            let antinode_b = Position {
                x: b.x + dx,
                y: b.y + dy,
            };
            if is_in_bounds(&antinode_b, map.dimension) {
                antinodes.insert(antinode_b.clone());
            }
        }
    }

    antinodes.len() as u32
}

fn is_in_bounds(antinode: &Position, dimension: u8) -> bool {
    antinode.x >= 0
        && antinode.x < dimension as i8
        && antinode.y >= 0
        && antinode.y < dimension as i8
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
