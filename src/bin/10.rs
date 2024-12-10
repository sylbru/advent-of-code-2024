#![allow(unused)]

use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Position {
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|char| char.parse::<u8>().ok())
                .collect()
        })
        .collect();
    println!("{:?}", map);

    let mut trailheads = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in (*line).iter().enumerate() {
            if *tile == 0 {
                trailheads.push(Position { x, y });
            }
        }
    }

    println!("{:?}", trailheads);

    Some(
        trailheads
            .iter()
            .map(|trailhead| find_reachable_summits(*trailhead, &map).len())
            .sum::<usize>()
            .try_into()
            .unwrap(),
    )
}

fn get_tile(map: &Vec<Vec<u8>>, position: Position) -> u8 {
    map[position.y][position.x]
}

fn find_reachable_summits(trailhead: Position, map: &Vec<Vec<u8>>) -> HashSet<Position> {
    let mut reachable_summits: HashSet<Position> = HashSet::new();
    let mut ongoing_trails: Vec<Position> = Vec::new();
    ongoing_trails.push(trailhead);

    while !ongoing_trails.is_empty() {
        step(&mut ongoing_trails, &map, &mut reachable_summits);
    }

    reachable_summits

    // start at position
    // in all directions, find reachable tiles
    // discard
    // maintain list of ongoing trails, start with four (start+up/down/left/right)
    // if a trail has no more reachable adjacent tiles, discard it
    // maintain list of reached summits
    // once a trail reaches a summit, add to reached summits
}

fn step(
    ongoing_trails: &mut Vec<Position>,
    map: &Vec<Vec<u8>>,
    reachable_summits: &mut HashSet<Position>,
) {
    let mut changes: Vec<(usize, Position)> = Vec::new();
    let mut new_ongoing_trails: Vec<Position> = Vec::new();

    for (index, position) in ongoing_trails.iter().enumerate() {
        // if right is available, add a position to that tile
        let position_right = Position {
            x: position.x + 1,
            y: position.y,
        };

        if in_bounds(position_right, map) {
            let tile_right = get_tile(map, position_right);

            if get_tile(map, *position) + 1 == tile_right {
                // changes.push((index, position_right));
                new_ongoing_trails.push(position_right);
            }

            if tile_right == 9 {
                (*reachable_summits).insert(position_right);
            }
        }

        // if left is available, add a position to that tile
        // if bottom is available, add a position to that tile
        // if up is available, add a position to that tile

        // remove the starting tile
    }

    *ongoing_trails = new_ongoing_trails;
}

fn in_bounds(position: Position, map: &Vec<Vec<u8>>) -> bool {
    let height = map.len();
    let width = map[0].len();

    position.x >= 0 && position.x < width && position.y >= 0 && position.y < height
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
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
