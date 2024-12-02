advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(run_one)
}

fn run_one(reports: Vec<Vec<u32>>) -> u32 {
    reports.iter().filter(is_safe).count() as u32
}

fn is_safe(report: &&Vec<u32>) -> bool {
    true
}

fn parse(input: &str) -> Option<Vec<Vec<u32>>> {
    let ints: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|level| level.parse::<u32>().ok())
                .collect()
        })
        .collect();

    Some(ints)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
