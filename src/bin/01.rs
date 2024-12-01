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
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    let pairs_str = input.lines().filter_map(|line| line.split_once("   "));

    for (a_str, b_str) in pairs_str {
        if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
            list_a.push(a);
            list_b.push(b);
        }
    }
    Some((list_a, list_b))
}

pub fn part_two(input: &str) -> Option<u32> {
    parse_one(input).map(run_two)
}

fn run_two((list_a, list_b): (Vec<i32>, Vec<i32>)) -> u32 {
    let mut similarity_score: u32 = 0;

    for a in list_a {
        let a_occurrences_in_b: u32 = list_b.iter().filter(|&&b| b == a).count() as u32;
        similarity_score += (a as u32) * a_occurrences_in_b;
    }

    similarity_score
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
        assert_eq!(result, Some(31));
    }
}
