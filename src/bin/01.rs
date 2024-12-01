advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    parse_one(input).map(run_one)
}

fn run_one((mut list_a, mut list_b): (Vec<i32>, Vec<i32>)) -> u32 {
    list_a.sort();
    list_b.sort();
    let pairs = list_a.iter().zip(list_b);
    let result = pairs.fold(0, |acc, (a, b)| acc + ((a - b).abs() as u32));
    result
}

fn parse_one(input: &str) -> Option<(Vec<i32>, Vec<i32>)> {
    let result: (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .filter_map(
            |(a_str, b_str)| match (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                (Ok(a), Ok(b)) => Some((a, b)),
                _ => None,
            },
        )
        .fold(
            (Vec::new(), Vec::new()),
            |(mut acc_a, mut acc_b), (a, b)| {
                acc_a.push(a);
                acc_b.push(b);
                (acc_a, acc_b)
            },
        );
    Some(result)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
