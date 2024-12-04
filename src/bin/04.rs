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
                if check_horizontal(&grid, x, y, width, height) {
                    count += 1;
                }
                if check_horizontal_backwards(&grid, x, y, width, height) {
                    count += 1;
                }
                if check_vertical(&grid, x, y, width, height) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

fn check_horizontal(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> bool {
    x <= width - 4 && grid[y][x + 1] == 'M' && grid[y][x + 2] == 'A' && grid[y][x + 3] == 'S'
}

fn check_horizontal_backwards(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> bool {
    x >= 3 && grid[y][x - 1] == 'M' && grid[y][x - 2] == 'A' && grid[y][x - 3] == 'S'
}

fn check_vertical(grid: &Vec<Vec<char>>, x: usize, y: usize, width: usize, height: usize) -> bool {
    y <= width - 4 && grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S'
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
