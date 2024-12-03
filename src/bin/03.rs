use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(run)
}

enum Instruction {
    Multiply(u32, u32),
    // Enable,
    // Disable,
}

fn parse(input: &str) -> Option<Vec<Instruction>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();

    for capture in re.captures_iter(input) {
        let a = capture.get(1).unwrap().as_str();
        let b = capture.get(2).unwrap().as_str();

        instructions.push(Instruction::Multiply(
            a.parse::<u32>().unwrap(),
            b.parse::<u32>().unwrap(),
        ));
    }

    Some(instructions)
}

fn parse_two(input: &str) -> Option<Vec<Instruction>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut enabled = true;

    for capture in re.captures_iter(input) {
        if let Some(full_match) = capture.get(0) {
            let matched_str = full_match.as_str();
            if matched_str == "do()" {
                enabled = true;
            } else if matched_str == "don't()" {
                enabled = false;
            } else if enabled {
                if let (Some(a), Some(b)) = (capture.get(1), capture.get(2)) {
                    if let (Ok(num_a), Ok(num_b)) =
                        (a.as_str().parse::<u32>(), b.as_str().parse::<u32>())
                    {
                        instructions.push(Instruction::Multiply(num_a, num_b));
                    }
                }
            }
        }
        // if capture.len() == 3 {
        //     if enabled {
        //         let a = capture.get(1).unwrap().as_str();
        //         let b = capture.get(2).unwrap().as_str();

        //         instructions.push(Instruction::Multiply(
        //             a.parse::<u32>().unwrap(),
        //             b.parse::<u32>().unwrap(),
        //         ));
        //     }
    }

    println!("{}", instructions.len());

    Some(instructions)
}

fn run(pairs: Vec<Instruction>) -> u32 {
    pairs.iter().fold(0, |acc, instruction| match instruction {
        Instruction::Multiply(a, b) => acc + a * b,
        // _ => unimplemented!("Enable/Disable"),
    })
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
