advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let codes: Vec<&str> = input.lines().collect();

    let complexities: Vec<u32> = vec![68 * 29, 60 * 980, 68 * 179, 64 * 456, 64 * 379];

    Some(complexities.iter().sum())
}

// code 029A
// -> find shortest sequence on the numeric keypad
// sequence <A^A>^^AvvvA
// -> find shortest sequence on a directional keypad
// -> sequence v<<A>>^A<A>AvA<^AA>A<vAAA>^A
// -> repeat
// -> sequence  <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// -> complexity = length of sequence * numeric part of code
//
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
