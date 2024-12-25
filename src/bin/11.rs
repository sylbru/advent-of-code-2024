use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> Option<Vec<usize>> {
    Some(
        input
            .split(" ")
            .map(|number_str| number_str.parse().unwrap())
            .collect(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).map(|stones| blink_stones(stones, 25))
}

pub fn part_two(input: &str) -> Option<u32> {
    parse(input).map(|stones| blink_stones(stones, 35))
}

fn blink_stones(stones: Vec<usize>, times: u8) -> u32 {
    let mut memo: HashMap<(usize, u8), u32> = HashMap::new();

    stones
        .iter()
        .map(|&stone| count_after_blinks_single_stone(stone, times, &mut memo))
        .sum()
}

fn count_after_blinks_single_stone(
    stone: usize,
    times: u8,
    memo: &mut HashMap<(usize, u8), u32>,
) -> u32 {
    let mut stones = vec![stone];

    if let Some(result) = memo.get(&(stone, times)) {
        println!("using memo! {} {} {}", stone, times, result);
        return *result;
    } else {
        for i in 0..times {
            stones = stones
                .iter()
                .map(|&stone| transform_stone(stone))
                .collect::<Vec<Vec<usize>>>()
                .concat();
            memo.insert((stone, i + 1), stones.len() as u32);
        }
    }

    stones.len() as u32
}

fn transform_stone(stone: usize) -> Vec<usize> {
    match stone {
        0 => vec![1],
        x => {
            let x_str = x.to_string();
            let x_str_len = x_str.len();
            if x_str_len % 2 == 0 {
                // split
                let (a_str, b_str) = x_str.split_at(x_str_len / 2);
                vec![a_str.parse().unwrap(), b_str.parse().unwrap()]
            } else {
                // multiply
                vec![x * 2024]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
