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
    rules
        .trim()
        .split("\n")
        .filter_map(|rule| {
            rule.split_once("|").map(|(page_a, page_b)| {
                (page_a.parse::<u8>().unwrap(), page_b.parse::<u8>().unwrap())
            })
        })
        .collect()
}

fn parse_updates(updates: &str) -> Vec<Vec<u8>> {
    updates
        .trim()
        .split("\n")
        .map(|update| {
            update
                .split(",")
                .map(|page_number| page_number.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn run_one((ordering_rules, updates): (Vec<(u8, u8)>, Vec<Vec<u8>>)) -> usize {
    let ordered: Vec<Vec<u8>> = updates
        .iter()
        .filter(|update| is_ordered(update, &ordering_rules))
        .cloned()
        .collect();

    ordered
        .iter()
        .map(|update| middle_page_number(update))
        .fold(0, |acc, page_number| acc + (page_number as usize))
}

fn is_ordered(update: &Vec<u8>, ordering_rules: &Vec<(u8, u8)>) -> bool {
    ordering_rules
        .iter()
        .all(|ordering_rule| is_respected(ordering_rule, update))
}

fn is_respected((page_a, page_b): &(u8, u8), update: &Vec<u8>) -> bool {
    let pos_a = update.iter().position(|p| p == page_a);
    let pos_b = update.iter().position(|p| p == page_b);

    match (pos_a, pos_b) {
        (None, _) | (_, None) => true,
        (Some(a), Some(b)) if a < b => true,
        _ => false,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    parse(input).map(run_one)
}

fn middle_page_number(update: &Vec<u8>) -> u8 {
    let middle_index = (update.len() - 1) / 2;
    update[middle_index]
}

fn run_two((ordering_rules, updates): (Vec<(u8, u8)>, Vec<Vec<u8>>)) -> usize {
    let unordered: Vec<Vec<u8>> = updates
        .iter()
        .filter(|update| !is_ordered(update, &ordering_rules))
        .cloned()
        .collect();

    unordered
        .iter()
        .map(|update| reorder(update, &ordering_rules))
        .map(|update| middle_page_number(&update))
        .fold(0, |acc, page_number| acc + (page_number as usize))
}

fn reorder(update: &Vec<u8>, ordering_rules: &Vec<(u8, u8)>) -> Vec<u8> {
    let mut update_copy = update.clone();
    let broken_rules: Vec<_> = ordering_rules
        .iter()
        .filter(|ordering_rule| !is_respected(ordering_rule, &update))
        .collect();

    for (page_a, page_b) in &broken_rules {
        let page_a_position = update.iter().position(|p| p == page_a).unwrap();
        let page_b_position = update.iter().position(|p| p == page_b).unwrap();
        update_copy.swap(page_a_position, page_b_position);
    }
    println!("original: {:?}", update);
    println!("broken: {:?}", broken_rules.clone());
    println!("fixed?: {:?}", update_copy);
    assert!(is_ordered(&update_copy, ordering_rules));
    update_copy
}

pub fn part_two(input: &str) -> Option<usize> {
    parse(input).map(run_two)
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
        assert_eq!(result, Some(123));
    }
}
