#![allow(unused)]

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn calculate_2000th_secret_number(initial: usize) -> usize {
    let mut next = initial;

    for i in 0..2000 {
        next = next_secret_number(next);
    }

    next
}

fn next_secret_number(secret_number: usize) -> usize {
    let step_1 = prune(mix(secret_number, secret_number * 64));
    let step_2 = prune(mix(step_1, step_1 / 32));
    let step_3 = prune(mix(step_2, step_2 * 2048));
    step_3
}

fn mix(secret_number: usize, value: usize) -> usize {
    value ^ secret_number
}

fn prune(secret_number: usize) -> usize {
    secret_number % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_next() {
        assert_eq!(next_secret_number(123), 15887950);
        assert_eq!(next_secret_number(16495136), 527345);
    }

    #[test]
    fn test_2000th_secret_number() {
        assert_eq!(calculate_2000th_secret_number(1), 8685429);
        assert_eq!(calculate_2000th_secret_number(2024), 8667524);
    }

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
