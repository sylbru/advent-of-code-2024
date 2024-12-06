advent_of_code::solution!(6);

#[derive(Debug)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
}

#[derive(Debug)]
enum Slot {
    Free,
    Busy,
    Visited,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse(input: &str) -> Option<(Vec<Vec<Slot>>, Guard)> {
    Some((
        vec![vec![Slot::Free]],
        Guard {
            position: (0, 0),
            direction: Direction::Up,
        },
    ))
}
pub fn part_one(input: &str) -> Option<isize> {
    parse(input).map(run_one)
}

fn run_one((mut grid, mut guard): (Vec<Vec<Slot>>, Guard)) -> isize {
    // let ref mut grid_ref = &grid;
    // let ref mut guard_ref = &guard;
    let mut visited_slots = 0;

    let desired_position = match guard.direction {
        Direction::Up => (guard.position.0, guard.position.1 - 1),
        Direction::Down => (guard.position.0, guard.position.1 + 1),
        Direction::Left => (guard.position.0 - 1, guard.position.1),
        Direction::Right => (guard.position.0 + 1, guard.position.1),
    };

    while desired_position.0 < 10
        && desired_position.0 >= 0
        && desired_position.1 < 10
        && desired_position.1 >= 0
    {
        match grid[desired_position.1 as usize][desired_position.0 as usize] {
            Slot::Free => {
                // set previous to visited
                grid[guard.position.1 as usize][guard.position.0 as usize] = Slot::Visited;
                // move guard
                guard.position = desired_position;
                // increment count
                visited_slots += 1;
            }
            Slot::Visited => guard.position = desired_position,
            Slot::Busy => guard.direction = to_the_right(guard.direction),
        }
        // tick(grid_ref, guard_ref, &mut visited_slots);
    }

    // out of the loop, means that the guard is now out of bounds
    // we need to add one to visited_slots (didn’t update the grid but who cares)
    visited_slots += 1;
    println!("{:?}", grid);
    println!("{:?}", guard);
    visited_slots
}

// fn tick(grid: &Vec<Vec<Slot>>, guard: &Guard, visited_slots: &isize) -> () {
//     let desired_position = match guard.direction {
//         Direction::Up => (guard.position.0, guard.position.1 - 1),
//         Direction::Down => (guard.position.0, guard.position.1 + 1),
//         Direction::Left => (guard.position.0 - 1, guard.position.1),
//         Direction::Right => (guard.position.0 + 1, guard.position.1),
//     };

//     match grid[desired_position.1][desired_position.0] {
//         Slot::Free => {
//             // set previous to visited
//             grid[guard.position.1][guard.position.0] = Slot::Visited;
//             // move guard
//             guard.position = desired_position;
//             // increment count
//             *visited_slots += 1;
//         }
//         Slot::Visited => guard.position = desired_position,
//         Slot::Busy => guard.direction = to_the_right(guard.direction),
//     }
// }

fn to_the_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
