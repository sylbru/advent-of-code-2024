advent_of_code::solution!(6);

#[derive(Debug)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
}

#[derive(Debug, PartialEq)]
enum Slot {
    Free,
    Busy,
    Visited,
}

impl From<char> for Slot {
    fn from(value: char) -> Self {
        match value {
            '.' | '^' => Self::Free,
            '#' => Self::Busy,
            'X' => Self::Visited,
            _ => panic!("Invalid character: {}", value),
        }
    }
}

// impl PartialEq for Slot {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) k
//     }
// }
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse(input: &str) -> Option<(Vec<Vec<Slot>>, Guard)> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|char| Slot::from(char)).collect())
        .collect();
    let guard = Guard {
        position: (
            input
                .lines()
                .find(|line| line.contains('^'))
                .unwrap()
                .char_indices()
                .find(|(_, c)| *c == '^')
                .unwrap()
                .0 as isize,
            input.lines().position(|line| line.contains("^")).unwrap() as isize,
        ),
        direction: Direction::Up,
    };

    Some((grid, guard))
}

pub fn part_one(input: &str) -> Option<isize> {
    parse(input).map(run_one)
}

fn run_one((mut grid, mut guard): (Vec<Vec<Slot>>, Guard)) -> isize {
    let mut visited_slots = 0;

    let mut desired_position;
    let dimension: isize = grid.len() as isize;

    loop {
        desired_position = position_ahead(&guard);

        if !(desired_position.0 < dimension
            && desired_position.0 >= 0
            && desired_position.1 < dimension
            && desired_position.1 >= 0)
        {
            // out of the loop, means that the guard is now out of bounds
            // we need to add one to visited_slots
            match grid[guard.position.1 as usize][guard.position.0 as usize] {
                Slot::Free => {
                    visited_slots += 1;
                    grid[guard.position.1 as usize][guard.position.0 as usize] = Slot::Visited;
                }
                _ => {}
            }
            break;
        }

        match grid[desired_position.1 as usize][desired_position.0 as usize] {
            Slot::Free | Slot::Visited => {
                match grid[guard.position.1 as usize][guard.position.0 as usize] {
                    Slot::Free => {
                        visited_slots += 1;
                        grid[guard.position.1 as usize][guard.position.0 as usize] = Slot::Visited
                    }
                    _ => {}
                }

                guard.position = desired_position;
            }
            Slot::Busy => guard.direction = to_the_right(guard.direction),
        }
    }

    // println!("{}", grid_to_string(&grid));
    visited_slots
}

fn position_ahead(guard: &Guard) -> (isize, isize) {
    match guard.direction {
        Direction::Up => (guard.position.0, guard.position.1 - 1),
        Direction::Down => (guard.position.0, guard.position.1 + 1),
        Direction::Left => (guard.position.0 - 1, guard.position.1),
        Direction::Right => (guard.position.0 + 1, guard.position.1),
    }
}

#[allow(dead_code)]
fn grid_to_string(grid: &Vec<Vec<Slot>>) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|slot| match slot {
                    Slot::Free => '.',
                    Slot::Busy => '#',
                    Slot::Visited => 'X',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn to_the_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
