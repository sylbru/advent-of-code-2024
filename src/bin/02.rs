advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(run_one)
}

fn run_one(reports: Vec<Vec<u32>>) -> u32 {
    reports.iter().filter(is_safe).count() as u32
}

fn run_two(reports: Vec<Vec<u32>>) -> u32 {
    reports.iter().filter(is_kinda_safe).count() as u32
}

enum Direction {
    Decreasing,
    Increasing,
    Invalid,
}

fn direction_from_adjacent_levels(first: u32, second: u32) -> Direction {
    if first < second {
        Direction::Increasing
    } else if first > second {
        Direction::Decreasing
    } else {
        Direction::Invalid
    }
}

fn is_safe(report: &&Vec<u32>) -> bool {
    let deltas: Vec<(Direction, u32)> = (*report)
        .windows(2)
        .map(|window| window_to_delta((window[0], window[1])))
        .collect();
    let all_increasing = deltas
        .iter()
        .all(|(direction, _)| matches!(direction, Direction::Increasing));
    let all_decreasing = deltas
        .iter()
        .all(|(direction, _)| matches!(direction, Direction::Decreasing));
    let all_gradual = deltas
        .iter()
        .all(|(_, difference)| *difference >= 1 && *difference <= 3);
    all_gradual && (all_increasing || all_decreasing)
}

fn is_kinda_safe(report: &&Vec<u32>) -> bool {
    let simplified_reports = generate_simplified_reports(report);
    simplified_reports.iter().any(|report| is_safe(&&report))
}

fn generate_simplified_reports(report: &&Vec<u32>) -> Vec<Vec<u32>> {
    let mut simplified_reports = Vec::new();

    for index in 0..report.len() {
        let simplified_report = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .map(|(_, &value)| value)
            .collect();
        simplified_reports.push(simplified_report);
    }

    simplified_reports
}

fn window_to_delta((first, second): (u32, u32)) -> (Direction, u32) {
    let direction = direction_from_adjacent_levels(first, second);
    let difference = first.abs_diff(second);
    (direction, difference)
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
    parse(input).map(run_two)
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
        assert_eq!(result, Some(4));
    }
}
