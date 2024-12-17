#![allow(unused)]

use std::{
    cmp::min,
    collections::{HashMap, HashSet, LinkedList},
    usize::MAX,
};

advent_of_code::solution!(16);

/*

parse empty positions into hashmap, and start and end positions
start direction : East
maintain list of potential paths : path with cost
    start with empty
at every step
    check tile in current direction
        if empty, add it to new potential paths with cost + 1


list of valid positions
find all lists of positions where each position differs to the next one by 1 in x or y
    build hashmaps for x and for y positions if necessary
then calculate costs for each one
keeping the current minimum in order to stop early if we’re already higher


*/

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse(input).unwrap();
    let mut paths: Vec<(LinkedList<Position>, usize, Direction)> = Vec::new();
    let mut best_path_score: usize = MAX;

    let mut start_path = LinkedList::new();
    start_path.push_back(parsed_input.start);
    paths.push((start_path, 0, Direction::East));

    for i in 0..6 {
        let mut new_paths: Vec<(LinkedList<Position>, usize, Direction)> = Vec::new();

        for (path, score, direction) in paths.iter_mut() {
            match &adjacent_positions_with_cost(
                path.back().unwrap(),
                &direction,
                &parsed_input.valid_positions,
            )[..]
            {
                [] => {}
                several_next_positions => {
                    for (next_position, new_direction, score_to_add) in
                        several_next_positions.iter()
                    {
                        if path.contains(next_position) {
                            continue;
                        }

                        let new_score = score.clone() + score_to_add;

                        if *next_position == parsed_input.end {
                            // We reached the end, let’s check if the score is better
                            // than what we have so far, and don’t keep exploring the path
                            best_path_score = min(best_path_score, new_score);
                        } else {
                            // Update score and direction, and keep exploring the path
                            let mut new_path = path.clone();
                            new_path.push_back(*next_position);
                            new_paths.push((
                                new_path,
                                score.clone() + score_to_add,
                                *new_direction,
                            ));
                        }
                    }
                }
            }
        }

        paths = new_paths;
        println!("{:?}", paths);
        //
        // break;
        // TODO exit condition
    }
    None
}

fn adjacent_positions_with_cost(
    path: &Position,
    direction: &Direction,
    valid_positions: &HashSet<Position>,
) -> Vec<(Position, Direction, usize)> {
    let to_east = valid_positions
        .get(&Position {
            x: path.x + 1,
            y: path.y,
        })
        .map(|pos| (pos, Direction::East));
    let to_west = valid_positions
        .get(&Position {
            x: path.x - 1,
            y: path.y,
        })
        .map(|pos| (pos, Direction::West));
    let to_south = valid_positions
        .get(&Position {
            x: path.x,
            y: path.y + 1,
        })
        .map(|pos| (pos, Direction::South));
    let to_north = valid_positions
        .get(&Position {
            x: path.x,
            y: path.y - 1,
        })
        .map(|pos| (pos, Direction::North));

    let options = vec![to_east, to_west, to_south, to_north];
    options
        .into_iter()
        .filter_map(|o| o.clone())
        .map(|(pos, new_direction)| {
            (
                *pos,
                new_direction.clone(),
                1 + rotation_cost(*direction, new_direction),
            )
        })
        .collect()
}

fn rotation_cost(from: Direction, to: Direction) -> usize {
    let changes = match (from, to) {
        (Direction::East, Direction::East)
        | (Direction::West, Direction::West)
        | (Direction::South, Direction::South)
        | (Direction::North, Direction::North) => 0,
        (Direction::East, Direction::South)
        | (Direction::South, Direction::East)
        | (Direction::East, Direction::North)
        | (Direction::North, Direction::East)
        | (Direction::West, Direction::South)
        | (Direction::South, Direction::West)
        | (Direction::West, Direction::North)
        | (Direction::North, Direction::West) => 1,
        (Direction::South, Direction::North)
        | (Direction::North, Direction::South)
        | (Direction::West, Direction::East)
        | (Direction::East, Direction::West) => 2,
    };

    changes * 1000
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Debug)]
struct Input {
    start: Position,
    end: Position,
    valid_positions: HashSet<Position>,
    valid_positions_for_x: HashMap<u8, Position>,
    valid_positions_for_y: HashMap<u8, Position>,
}

fn parse(input: &str) -> Option<Input> {
    let mut valid_positions: HashSet<Position> = HashSet::new();
    let mut valid_positions_for_x: HashMap<u8, Position> = HashMap::new();
    let mut valid_positions_for_y: HashMap<u8, Position> = HashMap::new();
    let mut end: Position = Position { x: 0, y: 0 };
    let mut start: Position = Position { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
            };

            match val {
                '.' => {
                    valid_positions.insert(position.clone());
                    valid_positions_for_x.insert(x as u8, position.clone());
                    valid_positions_for_y.insert(y as u8, position.clone());
                    println!("{:?}", position);
                }
                'E' => {
                    valid_positions.insert(position.clone());
                    end = position; // also add to valid positions ?
                }
                'S' => {
                    start = position;
                }
                _ => {}
            }
        }
    }

    Some(Input {
        start,
        end,
        valid_positions,
        valid_positions_for_x,
        valid_positions_for_y,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
