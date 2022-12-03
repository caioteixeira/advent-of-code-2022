use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = match args.len() {
        2 => args[1].as_str(),
        _ => "input.txt",
    };

    let input: Vec<String> = read_lines(filename).unwrap().flatten().collect();

    let total_score = compute_total_score(input);
    println!("Part 1: Total score is {}", total_score);
}

fn compute_total_score(input: Vec<String>) -> u32 {
    let mut score = 0;

    for line in input {
        let strategy = line.split_once(' ').unwrap();

        score += match strategy.1 {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        score += match strategy {
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            _ => 0,
        }
    }

    score
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::compute_total_score;

    #[test]
    fn compute_sample_score() {
        assert_eq!(
            15,
            compute_total_score(vec!(
                String::from("A Y"),
                String::from("B X"),
                String::from("C Z")
            ))
        )
    }

    #[test]
    fn compute_victories() {
        assert_eq!(
            24,
            compute_total_score(vec!(
                String::from("A Y"),
                String::from("B Z"),
                String::from("C X")
            ))
        )
    }

    #[test]
    fn compute_draw_scores() {
        assert_eq!(
            15,
            compute_total_score(vec!(
                String::from("A X"),
                String::from("B Y"),
                String::from("C Z")
            ))
        )
    }

    #[test]
    fn compute_defeat_scores() {
        assert_eq!(
            6,
            compute_total_score(vec!(
                String::from("A Z"),
                String::from("B X"),
                String::from("C Y")
            ))
        )
    }
}
