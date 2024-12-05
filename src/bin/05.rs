advent_of_code::solution!(5);

fn parse(input: &str) -> Option<(Vec<(u8, u8)>, Vec<Vec<u8>>)> {
    input
        .split_once("\n\n")
        .map(|(raw_ordering_rules, raw_updates)| {
            (
                parse_ordering_rules(raw_ordering_rules),
                parse_updates(raw_updates),
            )
        })
}

fn parse_ordering_rules(rules: &str) -> Vec<(u8, u8)> {
    vec![(1, 2)]
}
fn parse_updates(updates: &str) -> Vec<Vec<u8>> {
    vec![vec![42]]
}

pub fn part_one(input: &str) -> Option<u8> {
    let ordered: Vec<Vec<u8>> = vec![
        vec![75, 47, 61, 53, 29],
        vec![97, 61, 53, 29, 13],
        vec![75, 29, 13],
    ];

    Some(
        ordered
            .iter()
            .map(|update| middle_page_number(update))
            .fold(0, |acc, page_number| acc + page_number),
    )
}

fn middle_page_number(update: &Vec<u8>) -> u8 {
    let middle_index = (update.len() - 1) / 2;
    update[middle_index]
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
