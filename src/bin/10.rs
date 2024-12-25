#![allow(unused)]

use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Position {
    x: isize,
    y: isize,
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|char| char.parse::<u8>().ok())
                .collect()
        })
        .collect();

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

    Some(
        trailheads
            .iter()
            .map(|trailhead| find_reachable_summits(*trailhead, &map))
            // .map(|trailhead| find_reachable_summits(*trailhead, &map).len())
            .sum::<usize>()
            .try_into()
            .unwrap(),
    )
}

fn get_elevation(map: &Vec<Vec<u8>>, position: Position) -> u8 {
    // position is supposed to be in bounds
    map[position.y as usize][position.x as usize]
}

fn find_reachable_summits(trailhead: Position, map: &Vec<Vec<u8>>) -> usize {
    let mut reachable_summits: HashSet<Position> = HashSet::new();
    let mut ongoing_trails: Vec<Position> = Vec::new();
    let mut trails_count = 0;
    ongoing_trails.push(trailhead);

    while !ongoing_trails.is_empty() {
        step(
            &mut ongoing_trails,
            &map,
            &mut reachable_summits,
            &mut trails_count,
        );
    }

    // reachable_summits
    trails_count
}

fn step(
    ongoing_trails: &mut Vec<Position>,
    map: &Vec<Vec<u8>>,
    reachable_summits: &mut HashSet<Position>,
    trails_count: &mut usize,
) {
    let mut changes: Vec<(usize, Position)> = Vec::new();
    let mut new_ongoing_trails: Vec<Position> = Vec::new();

    for (index, position) in ongoing_trails.iter().enumerate() {
        let current_elevation = get_elevation(map, *position);

        explore_direction(
            to_right(*position),
            map,
            current_elevation,
            &mut new_ongoing_trails,
            reachable_summits,
            trails_count,
        );

        explore_direction(
            to_down(*position),
            map,
            current_elevation,
            &mut new_ongoing_trails,
            reachable_summits,
            trails_count,
        );

        explore_direction(
            to_left(*position),
            map,
            current_elevation,
            &mut new_ongoing_trails,
            reachable_summits,
            trails_count,
        );

        explore_direction(
            to_up(*position),
            map,
            current_elevation,
            &mut new_ongoing_trails,
            reachable_summits,
            trails_count,
        );
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
    trails_count: &mut usize,
) {
    if in_bounds(next_position, map) {
        let elevation = get_elevation(map, next_position);

        if from_elevation + 1 == elevation {
            if elevation == 9 {
                (*trails_count) += 1;
                // (*reachable_summits).insert(next_position);
            } else {
                new_ongoing_trails.push(next_position);
            }
        }
    }
}

fn in_bounds(position: Position, map: &Vec<Vec<u8>>) -> bool {
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    position.x >= 0 && position.x < width && position.y >= 0 && position.y < height
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
