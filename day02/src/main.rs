fn main() {
    let input = include_str!("input.txt");

    let input: Vec<&str> = input.lines().collect();

    let total_score = compute_total_score(&input);
    println!("Part 1: Total score is {}", total_score);

    let total_moves_score = compute_moves(&input);
    println!("Part 2: Total score is {}", total_moves_score);
}

fn compute_total_score(input: &Vec<&str>) -> u32 {
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

fn compute_moves(input: &Vec<&str>) -> u32 {
    let mut score = 0;

    for line in input {
        let strategy = line.split_once(' ').unwrap();

        let opponent_action: u32 = match strategy.0 {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("invalid action!"),
        };

        score += match strategy.1 {
            "X" => (opponent_action + 3 - 2) % 3 + 1,
            "Y" => 3 + opponent_action,
            "Z" => 6 + opponent_action % 3 + 1,
            _ => 0,
        };
        //println!("{} - {} -> {}", strategy.0, strategy.1, score);
    }

    score
}

#[cfg(test)]
mod tests {
    use crate::compute_moves;
    use crate::compute_total_score;

    #[test]
    fn compute_sample_score() {
        assert_eq!(15, compute_total_score(&vec!("A Y", "B X", "C Z")))
    }

    #[test]
    fn compute_victories() {
        assert_eq!(24, compute_total_score(&vec!("A Y", "B Z", "C X")))
    }

    #[test]
    fn compute_draw_scores() {
        assert_eq!(15, compute_total_score(&vec!("A X", "B Y", "C Z")))
    }

    #[test]
    fn compute_defeat_scores() {
        assert_eq!(6, compute_total_score(&vec!("A Z", "B X", "C Y")))
    }

    #[test]
    fn compute_sample_moves() {
        assert_eq!(12, compute_moves(&vec!("A Y", "B X", "C Z")))
    }

    #[test]
    fn compute_victory_moves() {
        assert_eq!(24, compute_moves(&vec!("A Z", "B Z", "C Z")))
    }

    #[test]
    fn compute_defeat_moves() {
        assert_eq!(6, compute_moves(&vec!("A X", "B X", "C X")))
    }

    #[test]
    fn compute_draw_moves() {
        assert_eq!(15, compute_moves(&vec!("A Y", "B Y", "C Y")))
    }
}
