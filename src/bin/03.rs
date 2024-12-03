use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    parse_one(input).map(run)
}

fn parse_one(input: &str) -> Option<Vec<(u32, u32)>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut instructions: Vec<(u32, u32)> = Vec::new();

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        instructions.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
    }

    Some(instructions)
}

fn parse_two(input: &str) -> Option<Vec<(u32, u32)>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut instructions = Vec::new();
    let mut enabled = true;

    for capture in re.captures_iter(input) {
        let matched_str = capture.get(0).map(|m| m.as_str());

        match matched_str {
            Some("do()") => enabled = true,
            Some("don't()") => enabled = false,
            _ if enabled => {
                if let (Ok(a), Ok(b)) = (capture[1].parse::<u32>(), capture[2].parse::<u32>()) {
                    instructions.push((a, b));
                }
            }
            _ => {}
        }
    }

    Some(instructions)
}

fn run(pairs: Vec<(u32, u32)>) -> u32 {
    pairs.iter().fold(0, |acc, (a, b)| acc + a * b)
}

pub fn part_two(input: &str) -> Option<u32> {
    parse_two(input).map(run)
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
        assert_eq!(result, Some(48));
    }
}
