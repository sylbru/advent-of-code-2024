advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    let distances = [2, 1, 0, 1, 2, 5];
    let result = distances.iter().fold(0, |acc, x| acc + x);
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

// sort both lists
// pair numbers (zip)
// list of pairs to list of distances
// sum the whole thing
// (or compute and sum the distances in one pass)

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
