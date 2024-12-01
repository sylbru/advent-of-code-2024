advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    let mut list_a = [3i32, 4, 2, 1, 3, 3];
    list_a.sort();
    let mut list_b = [4i32, 3, 5, 3, 9, 3];
    list_b.sort();
    let pairs = list_a.iter().zip(list_b);
    let result = pairs.fold(0, |acc, (a, b)| acc + ((a - b).abs() as u32));

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
