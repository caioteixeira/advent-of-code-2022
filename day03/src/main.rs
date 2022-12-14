use std::collections::HashSet;

fn main() {
    let input = parse_input(include_str!("input.txt"));

    let priorities_sum = sum_priority_of_duplicated_items(&input);
    println!("Part 1: Sum of priorities is {}", priorities_sum);

    let badge_priorities_sum = sum_priority_of_badges(&input);
    println!(
        "Part 2: Sum of badge priorities is {}",
        badge_priorities_sum
    )
}

fn parse_input(input: &str) -> Vec<&str> {
    let input: Vec<&str> = input.lines().collect();
    input
}

fn sum_priority_of_duplicated_items(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| find_duplicated_char(line))
        .map(convert_to_priority)
        .sum()
}

fn sum_priority_of_badges(input: &[&str]) -> u32 {
    input.chunks(3).map(find_badge_priority).sum()
}

fn find_badge_priority(group: &[&str]) -> u32 {
    let intersection = group
        .iter()
        // Creates a hash set for chars in  each line
        .map(|rucksack| rucksack.chars().to_owned().collect::<HashSet<char>>())
        // Get the items that are present in all the lines
        .reduce(|intersection, rucksack| intersection.intersection(&rucksack).cloned().collect());
    match intersection {
        Some(items) => convert_to_priority(*items.iter().next().unwrap()),
        None => unreachable!(),
    }
}

fn convert_to_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,
        _ => unreachable!(),
    }
}

fn find_duplicated_char(line: &str) -> char {
    let (first_half, second_half) = line.split_at(line.len() / 2);
    let items = first_half.chars().collect::<HashSet<char>>();
    second_half
        .chars()
        .find(|item| items.contains(item))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::convert_to_priority;
    use crate::find_duplicated_char;
    use crate::parse_input;
    use crate::sum_priority_of_badges;
    use crate::sum_priority_of_duplicated_items;

    #[test]
    fn test_find_duplicated_char() {
        assert_eq!('p', find_duplicated_char("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(
            'L',
            find_duplicated_char("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
        );
        assert_eq!('P', find_duplicated_char("PmmdzqPrVvPwwTWBwg"));
        assert_eq!('v', find_duplicated_char("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"))
    }

    #[test]
    fn test_convert_to_priority() {
        assert_eq!(16, convert_to_priority('p'));
        assert_eq!(38, convert_to_priority('L'));
        assert_eq!(22, convert_to_priority('v'));
    }

    #[test]
    fn test_sum_priorities_of_duplicated_items() {
        let input = parse_input(include_str!("test_input.txt"));
        assert_eq!(157, sum_priority_of_duplicated_items(&input));
    }

    #[test]
    fn test_sum_of_badge_priorities() {
        let input = parse_input(include_str!("test_input.txt"));
        assert_eq!(70, sum_priority_of_badges(&input));
    }
}
