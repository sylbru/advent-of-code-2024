advent_of_code::solution!(11);

fn parse(input: &str) -> Option<Vec<usize>> {
    Some(
        input
            .split(" ")
            .map(|number_str| number_str.parse().unwrap())
            .collect(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let stones = parse(input).unwrap();
    None
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
