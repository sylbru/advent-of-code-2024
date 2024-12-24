#![allow(unused)]

use std::collections::HashMap;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Gate {
    a: String,
    operator: Operator,
    b: String,
    to_wire: String,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

fn parse(input: &str) -> Option<(Vec<(&str, bool)>, Vec<Gate>)> {
    input.split_once("\n\n").map(|(part1, part2)| {
        println!("{:?}", parse_gates(part2));
        (parse_inputs(part1), parse_gates(part2))
    })
}

fn parse_inputs(inputs: &str) -> Vec<(&str, bool)> {
    inputs
        .lines()
        .map(|line| {
            let tuple = line.split_once(": ").unwrap();
            (tuple.0, tuple.1 == "1")
        })
        .collect()
}

fn parse_gates(gates: &str) -> Vec<Gate> {
    gates
        .lines()
        .map(|gate| match gate.split(" ").collect::<Vec<&str>>()[..] {
            [a, operator, b, "->", to_wire] => Gate {
                a: a.into(),
                operator: parse_operator(operator),
                b: b.into(),
                to_wire: to_wire.into(),
            },
            _ => panic!("can’t match"),
        })
        .collect()
}

fn parse_operator(operator: &str) -> Operator {
    match operator {
        "AND" => Operator::And,
        "OR" => Operator::Or,
        "XOR" => Operator::Xor,
        _ => panic!("wrong operator"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(run_one)
}

fn run_one((inputs, gates_): (Vec<(&str, bool)>, Vec<Gate>)) -> u32 {
    let mut values: HashMap<String, bool> = HashMap::new();
    let mut gates = gates_.clone();

    for input in inputs {
        values.insert(input.0.to_owned(), input.1);
    }

    while !gates.is_empty() {
        for gate in &gates {
            compute_gate(gate, &mut values);
        }

        println!("{:?}", values);
        // just one step for now
        panic!();
    }

    3214
}

fn compute_gate(gate: &Gate, values: &mut HashMap<String, bool>) -> () {
    let result = match gate.operator {
        Operator::And => *values.get(&gate.a).unwrap() && *values.get(&gate.b).unwrap(),
        Operator::Or => *values.get(&gate.a).unwrap() || *values.get(&gate.b).unwrap(),
        Operator::Xor => *values.get(&gate.a).unwrap() != *values.get(&gate.b).unwrap(),
    };

    values.insert(gate.to_wire.clone(), result);
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
