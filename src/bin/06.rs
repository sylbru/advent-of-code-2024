use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
}

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn run_one((grid, guard): (Vec<Vec<Slot>>, Guard)) -> isize {
    match run_loop((grid, guard)) {
        RouteResult::Finished(visited) => visited,
        RouteResult::Loops => panic!("Should not loop in part one!"),
    }
}

fn run_two((grid, guard): (Vec<Vec<Slot>>, Guard)) -> isize {
    let mut efficient_obstructions = 0;
    let dimension: isize = grid.len() as isize;

    for y in 0..dimension {
        for x in 0..dimension {
            if grid[y as usize][x as usize] == Slot::Free {
                let mut grid_copy = grid.clone();
                grid_copy[y as usize][x as usize] = Slot::Busy;

                if let RouteResult::Loops = run_loop((grid_copy, guard.clone())) {
                    efficient_obstructions += 1;
                }
            }
        }
    }
    efficient_obstructions
}

#[derive(Debug, PartialEq)]
enum RouteResult {
    Finished(isize), // Finished with a number of visited slots
    Loops,           // Ends up in a loop
}
fn run_loop((mut grid, mut guard): (Vec<Vec<Slot>>, Guard)) -> RouteResult {
    let mut visited_slots = 0;

    let mut desired_position;
    let dimension: isize = grid.len() as isize;
    let mut history: HashSet<((isize, isize), Direction)> = HashSet::new();

    loop {
        if history.contains(&(guard.position, guard.direction)) {
            return RouteResult::Loops;
        }

        history.insert((guard.position, guard.direction));

        // Mark current position as visited straight away if it’s not already
        if grid[guard.position.1 as usize][guard.position.0 as usize] == Slot::Free {
            grid[guard.position.1 as usize][guard.position.0 as usize] = Slot::Visited;
            visited_slots += 1;
        }

        desired_position = position_ahead(&guard);

        if !is_position_in_bounds(desired_position, dimension) {
            // out of the loop, means that the guard is now out of bounds
            // we need to add one to visited_slots
            break;
        }

        match grid[desired_position.1 as usize][desired_position.0 as usize] {
            Slot::Free | Slot::Visited => guard.position = desired_position,
            Slot::Busy => guard.direction = to_the_right(guard.direction),
        }
    }

    RouteResult::Finished(visited_slots)
}

fn is_position_in_bounds((x, y): (isize, isize), dimension: isize) -> bool {
    x >= 0 && x < dimension && y >= 0 && y < dimension
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

pub fn part_two(input: &str) -> Option<isize> {
    parse(input).map(run_two)
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
    fn test_loop() {
        let mut grid = vec![
            vec![Slot::Free, Slot::Busy, Slot::Free],
            vec![Slot::Free, Slot::Free, Slot::Busy],
            vec![Slot::Busy, Slot::Free, Slot::Free],
            vec![Slot::Free, Slot::Busy, Slot::Free],
        ];
        let mut guard = Guard {
            position: (1, 2),
            direction: Direction::Up,
        };
        let result = run_loop((grid, guard));
        assert_eq!(result, RouteResult::Loops);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
