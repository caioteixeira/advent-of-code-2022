#[derive(Clone, Debug, Copy)]
struct SectionRange {
    start: u32,
    end: u32,
}

#[derive(Clone, Debug, Copy)]
struct AssigmentPair {
    elf1: SectionRange,
    elf2: SectionRange,
}

impl SectionRange {
    fn from_string(input: &str) -> Option<SectionRange> {
        let (start_str, end_str) = input.split_once('-')?;
        let start: u32 = start_str.parse::<u32>().unwrap();
        let end: u32 = end_str.parse::<u32>().unwrap();

        Some(SectionRange { start, end })
    }

    fn fully_contains(&self, other: &SectionRange) -> bool {
        other.start >= self.start && other.end <= self.end
    }
}

impl AssigmentPair {
    fn from_string(input: &str) -> Option<AssigmentPair> {
        let (elf1_str, elf2_str) = input.split_once(',')?;

        let elf1 = SectionRange::from_string(elf1_str)?;
        let elf2 = SectionRange::from_string(elf2_str)?;

        Some(AssigmentPair { elf1, elf2 })
    }

    fn is_one_fully_contained(&self) -> bool {
        self.elf1.fully_contains(&self.elf2) || self.elf2.fully_contains(&self.elf1)
    }
}

fn main() {
    let priorities_sum = how_many_fully_contains(include_str!("input.txt"));
    println!("Part 1: Sum of priorities is {}", priorities_sum);
}

fn parse_input(input: &str) -> Vec<AssigmentPair> {
    let input: Vec<&str> = input.lines().collect();

    let mut output = Vec::new();
    for line in input {
        let pair = AssigmentPair::from_string(line);
        output.push(pair.unwrap());
    }

    output
}

fn how_many_fully_contains(input: &str) -> u32 {
    let pairs = parse_input(input);

    pairs
        .iter()
        .filter(|pair| pair.is_one_fully_contained())
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::how_many_fully_contains;

    #[test]
    fn test_fully_contained() {
        assert_eq!(2, how_many_fully_contains(include_str!("test_input.txt")));
    }
}
