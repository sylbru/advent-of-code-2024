advent_of_code::solution!(4);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse(input);
    let mut count = 0;
    let width: usize = grid.get(0).unwrap().len();
    let height: usize = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'X' {
                count += 1;
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
