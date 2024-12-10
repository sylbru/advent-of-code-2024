advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|char| char.parse::<u8>().ok())
                .collect()
        })
        .collect();
    println!("{:?}", map);
    None
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
