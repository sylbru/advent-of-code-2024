advent_of_code::solution!(7);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    println!("{:?}", parse(input));
    parse(input).map(run_one)
}

fn run_one(tests: Vec<(usize, Vec<usize>)>) -> usize {
    tests
        .iter()
        .filter(|one_test| is_valid(one_test, vec![Operator::Addition, Operator::Multiplication]))
        .map(|test| test.0)
        .sum()
}

fn run_two(tests: Vec<(usize, Vec<usize>)>) -> usize {
    tests
        .iter()
        .filter(|one_test| {
            is_valid(
                one_test,
                vec![
                    Operator::Addition,
                    Operator::Multiplication,
                    Operator::Concatenation,
                ],
            )
        })
        .map(|test| test.0)
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

fn is_valid((result, operands): &(usize, Vec<usize>), operators: Vec<Operator>) -> bool {
    let operator_combinations =
        itertools::repeat_n(operators, operands.len() - 1).multi_cartesian_product();

    for operators in operator_combinations {
        if *result == apply_operators_to_operands(operators, operands) {
            return true;
        }
    }

    return false;
}

fn apply_operators_to_operands(operators: Vec<Operator>, operands: &Vec<usize>) -> usize {
    let (start_value, remaining_operands) = operands.split_at(1);
    remaining_operands
        .iter()
        .zip(operators)
        .fold(start_value[0], |acc, (operand, operator)| match operator {
            Operator::Addition => acc + operand,
            Operator::Multiplication => acc * operand,
            Operator::Concatenation => {
                let mut acc_string = acc.to_string();
                acc_string.push_str(&operand.to_string());
                acc_string.parse().ok().unwrap()
            }
        })
}

fn parse(input: &str) -> Option<Vec<(usize, Vec<usize>)>> {
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

pub fn part_two(input: &str) -> Option<usize> {
    parse(input).map(run_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        let acc = 123;
        let operand = 456;
        let mut acc_string = acc.to_string();
        acc_string.push_str(&operand.to_string());
        assert_eq!(123456, acc_string.parse().ok().unwrap());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
