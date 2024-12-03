use std::default;

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(run_one)
}

enum Instruction {
    Multiply(u32, u32),
    Enable,
    Disable,
}

fn parse(input: &str) -> Option<Vec<Instruction>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        instructions.push(Instruction::Multiply(
            a.parse::<u32>().unwrap(),
            b.parse::<u32>().unwrap(),
        ));
    }

    Some(instructions)
}

fn run_one(pairs: Vec<Instruction>) -> u32 {
    pairs.iter().fold(0, |acc, instruction| match instruction {
        Instruction::Multiply(a, b) => acc + a * b,
        default => unimplemented!("Enable/Disable"),
    })
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
