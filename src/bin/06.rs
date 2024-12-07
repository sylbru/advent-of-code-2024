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

fn run_one((grid, guard): (Vec<Vec<Slot>>, Guard)) -> isize {
    match run_loop((grid, guard)) {
        RouteResult::Finished(visited) => visited,
        RouteResult::Loops => panic!("Should not loop in part one!"),
    }
}

fn run_two((grid, guard): (Vec<Vec<Slot>>, Guard)) -> isize {
    let returned = run_loop((grid, guard));
    42
}

enum RouteResult {
    Finished(isize), // Finished with a number of visited slots
    Loops,           // Ends up in a loop
}
fn run_loop((mut grid, mut guard): (Vec<Vec<Slot>>, Guard)) -> RouteResult {
    let mut visited_slots = 0;

    let mut desired_position;
    let dimension: isize = grid.len() as isize;

    loop {
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

    // println!("{}", grid_to_string(&grid));
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
