use std::collections::HashMap;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
}

// #[derive(Debug)]
// enum Direction {
//     East,
//     South,
//     West,
//     North,
// }

#[derive(Debug)]
struct Input {
    start: Position, // (Position, Direction),
    end: Position,
    valid_positions_for_x: HashMap<u8, Position>,
    valid_positions_for_y: HashMap<u8, Position>,
}

fn parse(input: &str) -> Option<Input> {
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
        valid_positions_for_x,
        valid_positions_for_y,
    })
}
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
