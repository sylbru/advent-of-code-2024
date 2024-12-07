advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    println!("{:?}", parse(input));
    parse(input).map(run_one)
}

fn run_one(tests: Vec<(u32, Vec<u32>)>) -> u32 {
    42
}

fn parse(input: &str) -> Option<Vec<(u32, Vec<u32>)>> {
    Some(
        input
            .lines()
            .map(|line| line.split_once(":").unwrap())
            .map(|(result, operands_str)| {
                (
                    result.parse().unwrap(),
                    operands_str
                        .split(" ")
                        .filter_map(|operand| operand.parse().ok())
                        .collect(),
                )
            })
            .collect(),
    )
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
