#![allow(unused)]

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let codes: Vec<&str> = input.lines().collect();

    let complexities = codes.iter().map(|code| complexity(code));

    Some(complexities.sum())
}

fn complexity(code: &str) -> u32 {
    let numeric_part: u32 = code[..code.len() - 1].parse().ok().unwrap();
    let sequence = dirpad_to_dirpad(dirpad_to_dirpad(numpad_to_dirpad(code)));

    // // if code == "039A" {
    // println!("{}", numpad_to_dirpad(code));
    // println!("{}", dirpad_to_dirpad(numpad_to_dirpad(code)));
    // println!(
    //     "{}",
    //     dirpad_to_dirpad(dirpad_to_dirpad(numpad_to_dirpad(code)))
    // );
    // // }
    println!(
        "{}: {} * {} (len of {})",
        code,
        numeric_part,
        sequence.len(),
        sequence
    );
    numeric_part * sequence.len() as u32
}

fn numpad_to_dirpad(code: &str) -> String {
    let mut current = 'A';
    let mut y = row_for_char(current);
    let mut x = col_for_char(current);
    let mut sequence = "".to_owned();

    for target in code.chars() {
        // move to char
        let dy: i8 = row_for_char(target) - y;
        let dx: i8 = col_for_char(target) - x;

        if dx < 0 && dy < 0 {
            // going NW, avoid hole
            // move vertically then horizontally
            move_vert(&mut sequence, dy);
            move_horiz(&mut sequence, dx);
        } else {
            // general case
            // move horizontally then vertically
            move_horiz(&mut sequence, dx);
            move_vert(&mut sequence, dy);
        }

        x = x + dx;
        y = y + dy;
        current = numpad()[y as usize][x as usize];

        sequence.push('A');
    }

    sequence
    // <A^A>^^AvvvA
}

fn move_horiz(sequence: &mut String, dx: i8) -> () {
    let count_horiz = dx.abs() as usize;
    let horiz_move = if dx < 0 { "<" } else { ">" };
    sequence.push_str(&horiz_move.repeat(count_horiz));
}

fn move_vert(sequence: &mut String, dy: i8) -> () {
    let count_vert = dy.abs() as usize;
    let vert_move = if dy < 0 { "^" } else { "v" };
    sequence.push_str(&vert_move.repeat(count_vert));
}

fn dirpad_to_dirpad(moves: String) -> String {
    let mut current = 'A';
    let mut y = row_for_char_dirpad(&current);
    let mut x = col_for_char_dirpad(&current);
    let mut sequence = "".to_owned();

    for target in moves.chars() {
        // move to char
        let dy: i8 = row_for_char_dirpad(&target) - y;
        let dx: i8 = col_for_char_dirpad(&target) - x;

        if dx < 0 && dy < 0 {
            // going SW, avoid hole
            // move vertically then horizontally
            move_vert(&mut sequence, dy);
            move_horiz(&mut sequence, dx);
        } else {
            // general case
            // move horizontally then vertically
            move_horiz(&mut sequence, dx);
            move_vert(&mut sequence, dy);
        }

        x = x + dx;
        y = y + dy;
        current = dirpad()[y as usize][x as usize];

        sequence.push('A');
    }

    sequence
}

fn dirpad() -> Vec<Vec<char>> {
    vec![vec!['⌧', '^', 'A'], vec!['<', 'v', '>']]
}

fn row_for_char_dirpad(current: &char) -> i8 {
    dirpad()
        .iter()
        .position(|row| row.contains(current))
        .unwrap()
        .try_into()
        .unwrap()
}

fn col_for_char_dirpad(current: &char) -> i8 {
    dirpad()
        .iter()
        .find(|row| row.contains(current))
        .unwrap()
        .iter()
        .position(|character| character == current)
        .unwrap()
        .try_into()
        .unwrap()
}

fn numpad() -> Vec<Vec<char>> {
    vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['⌧', '0', 'A'],
    ]
}

fn row_for_char(character: char) -> i8 {
    match character {
        '7' | '8' | '9' => 0,
        '4' | '5' | '6' => 1,
        '1' | '2' | '3' => 2,
        '0' | 'A' => 3,
        _ => panic!("can’t find row for char {}", character),
    }
}

fn col_for_char(character: char) -> i8 {
    match character {
        '7' | '4' | '1' => 0,
        '8' | '5' | '2' | '0' => 1,
        '9' | '6' | '3' | 'A' => 2,
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

    #[test]
    fn test_numpad_0A() {
        assert_eq!(numpad_to_dirpad("0A"), "<A>A");
    }

    #[test]
    fn test_numpad_02A() {
        assert_eq!(numpad_to_dirpad("02A"), "<A^A>vA");
    }

    #[test]
    fn test_numpad_to_dirpad() {
        assert_eq!(numpad_to_dirpad("029A"), "<A^A>^^AvvvA");
    }

    #[test]
    fn test_dirpad_to_dirpad() {
        assert_eq!(
            dirpad_to_dirpad("<A^A>^^AvvvA".to_owned()),
            "<<vA>>^A<A>AvA^<AA>A<vAAA>^A" // "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
        );
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
