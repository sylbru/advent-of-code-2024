#![allow(unused)]

use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Position {
    x: isize,
    y: isize,
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
                trailheads.push(Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
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

fn get_elevation(map: &Vec<Vec<u8>>, position: Position) -> u8 {
    // position is supposed to be in bounds
    map[position.y as usize][position.x as usize]
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
        explore_direction(
            to_right(*position),
            map,
            get_elevation(map, *position),
            &mut new_ongoing_trails,
            reachable_summits,
        );

        explore_direction(
            to_down(*position),
            map,
            get_elevation(map, *position),
            &mut new_ongoing_trails,
            reachable_summits,
        );

        explore_direction(
            to_left(*position),
            map,
            get_elevation(map, *position),
            &mut new_ongoing_trails,
            reachable_summits,
        );

        explore_direction(
            to_up(*position),
            map,
            get_elevation(map, *position),
            &mut new_ongoing_trails,
            reachable_summits,
        );

        // if bottom is available, add a position to that tile
        // if up is available, add a position to that tile

        // remove the starting tile
    }

    *ongoing_trails = new_ongoing_trails;
}

fn to_right(position: Position) -> Position {
    Position {
        x: position.x + 1,
        y: position.y,
    }
}

fn to_down(position: Position) -> Position {
    Position {
        x: position.x,
        y: position.y + 1,
    }
}

fn to_left(position: Position) -> Position {
    Position {
        x: position.x - 1,
        y: position.y,
    }
}

fn to_up(position: Position) -> Position {
    Position {
        x: position.x,
        y: position.y - 1,
    }
}
fn explore_direction(
    next_position: Position,
    map: &Vec<Vec<u8>>,
    from_elevation: u8,
    new_ongoing_trails: &mut Vec<Position>,
    reachable_summits: &mut HashSet<Position>,
) {
    if in_bounds(next_position, map) {
        let elevation = get_elevation(map, next_position);

        if from_elevation + 1 == elevation {
            new_ongoing_trails.push(next_position);
        }

        if elevation == 9 {
            (*reachable_summits).insert(next_position);
        }
    }
}

fn in_bounds(position: Position, map: &Vec<Vec<u8>>) -> bool {
    let height = map.len() as isize;
    let width = map[0].len() as isize;

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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
