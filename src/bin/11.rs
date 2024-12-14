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
    let stones = parse(input).unwrap();

    Some(blink(stones, 25))
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

fn blink(stones: Vec<usize>, times: u8) -> u32 {
    if (times > 0) {
        let new_stones: Vec<usize> = stones
            .iter()
            .map(|&stone| transform_stone(stone))
            .collect::<Vec<Vec<usize>>>()
            .concat();
        blink(new_stones, times - 1)
    } else {
        println!("{:?}", stones);
        stones.len() as u32
    }
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
