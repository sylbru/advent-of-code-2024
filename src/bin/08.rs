#![allow(unused)]

use std::collections::HashMap;
advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i8,
    y: i8,
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
    println!("{:?}", antennas_positions_by_frequency);
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
