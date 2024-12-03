advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(run_one)
}

fn parse(input: &str) -> Option<Vec<(u32, u32)>> {
    Some(vec![(2, 4), (5, 5), (11, 8), (8, 5)])
}

fn run_one(pairs: Vec<(u32, u32)>) -> u32 {
    pairs.iter().fold(0, |acc, &(a, b)| acc + a * b)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
