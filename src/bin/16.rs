#![allow(unused)]

use std::collections::{HashMap, HashSet, LinkedList};

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
    let mut best_path_score: usize;

    let mut start_path = LinkedList::new();
    start_path.push_back(parsed_input.start);
    paths.push((start_path, 0, Direction::East));

    for i in 0..2 {
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
                    for (next_position, score_to_add) in several_next_positions.iter() {
                        if path.contains(next_position) {
                            continue;
                        }

                        let mut new_path = path.clone();
                        new_path.push_back(*next_position);
                        new_paths.push((new_path, score.clone() + score_to_add, *direction));
                        // compute new score
                    }
                }
            }
        }

        paths = new_paths;
        println!("{:?}", paths);
        // keep direction in order to accurately compute scores
        // compute score
        // break;
        // TODO exit condition
    }
    None
}

fn adjacent_positions_with_cost(
    path: &Position,
    direction: &Direction,
    valid_positions: &HashSet<Position>,
) -> Vec<(Position, usize)> {
    let to_right = valid_positions
        .get(&Position {
            x: path.x + 1,
            y: path.y,
        })
        .map(|pos| (pos, Direction::East));
    let to_left = valid_positions
        .get(&Position {
            x: path.x - 1,
            y: path.y,
        })
        .map(|pos| (pos, Direction::West));
    let to_down = valid_positions
        .get(&Position {
            x: path.x,
            y: path.y + 1,
        })
        .map(|pos| (pos, Direction::South));
    let to_up = valid_positions
        .get(&Position {
            x: path.x,
            y: path.y - 1,
        })
        .map(|pos| (pos, Direction::North));

    let options = vec![to_right, to_left, to_down, to_up];
    options
        .into_iter()
        .filter_map(|o| o.clone())
        .map(|(pos, new_direction)| (*pos, 1 + rotation_cost(*direction, new_direction)))
        .collect()
}

fn rotation_cost(from: Direction, to: Direction) -> usize {
    let changes = match (from, to) {
        (Direction::East, Direction::East) => 0,
        (Direction::East, Direction::South) => 1,
        (Direction::East, Direction::West) => 2,
        (Direction::East, Direction::North) => 1,
        (Direction::South, Direction::East) => 1,
        (Direction::South, Direction::South) => 0,
        (Direction::South, Direction::West) => 1,
        (Direction::South, Direction::North) => 2,
        (Direction::West, Direction::East) => 2,
        (Direction::West, Direction::South) => 1,
        (Direction::West, Direction::West) => 0,
        (Direction::West, Direction::North) => 1,
        (Direction::North, Direction::East) => 1,
        (Direction::North, Direction::South) => 2,
        (Direction::North, Direction::West) => 1,
        (Direction::North, Direction::North) => 0,
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
