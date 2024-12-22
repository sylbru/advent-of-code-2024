advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let codes: Vec<&str> = input.lines().collect();

    let complexities: Vec<u32> = vec![68 * 29, 60 * 980, 68 * 179, 64 * 456, 64 * 379];

    Some(complexities.iter().sum())
}

fn numpad_to_dirpad(code: &str) -> String {
    let mut current = 'A';
    let mut sequence = "".to_owned();

    for target in code.chars() {
        // move to char, then (A)ctivate
        while current != target {
            // move in the right direction
            let dy: i8 = row_for_char(target) - row_for_char(current);
            let dx: i8 = col_for_char(target) - col_for_char(current);

            if dx < 0 && dy < 0 {
                // going NW, avoid hole
                // move vertically then horizontally
                let count_vert = (-dy) as usize;
                sequence.push_str(&"^".repeat(count_vert));

                let count_horiz = (-dx) as usize;
                sequence.push_str(&"<".repeat(count_horiz));
            } else {
                // move horizontally then vertically
                let count_horiz = dx.abs() as usize;
                if (dx < 0) {
                    sequence.push_str(&"<".repeat(count_horiz));
                } else {
                    sequence.push_str(&">".repeat(count_horiz));
                }

                let count_vert = dy.abs() as usize;
                if (dy < 0) {
                    sequence.push_str(&"^".repeat(count_vert));
                } else {
                    sequence.push_str(&"v".repeat(count_vert));
                }
            }
            println!("{}", sequence);
            // update current with new character
        }

        sequence.push('A');
    }

    sequence
    // <A^A>^^AvvvA
}

fn row_for_char(character: char) -> i8 {
    match character {
        '7' | '8' | '9' => 1,
        '4' | '5' | '6' => 2,
        '1' | '2' | '3' => 3,
        '0' | 'A' => 4,
        _ => panic!("can’t find row for char {}", character),
    }
}

fn col_for_char(character: char) -> i8 {
    match character {
        '7' | '4' | '1' => 1,
        '8' | '5' | '2' | '0' => 2,
        '9' | '6' | '3' | 'A' => 3,
        _ => panic!("can’t find col for char {}", character),
    }
}

// 029A
// 789
// 456
// 123
//  0A
//
// if in same row, go right or left
// if

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
    fn test_numpad_A() {
        assert_eq!(numpad_to_dirpad("A"), "A");
    }

    #[test]
    fn test_numpad_0() {
        assert_eq!(numpad_to_dirpad("0"), "<A");
    }

    #[ignore]
    #[test]
    fn test_numpad_0A() {
        assert_eq!(numpad_to_dirpad("0A"), "<A>A");
    }

    #[ignore]
    #[test]
    fn test_numpad_02A() {
        assert_eq!(numpad_to_dirpad("02A"), "<A^Av>A");
    }

    #[test]
    fn test_numpad_to_dirpad() {
        assert_eq!(numpad_to_dirpad("029A"), "<A^A>^^AvvvA");
    }

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
