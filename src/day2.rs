use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    fn from_intended_outcome(other_choice: RPSChoice, intended_outcome: Outcome) -> Self {
        match (other_choice, intended_outcome) {
            (RPSChoice::Rock, Outcome::Win) => Self::Paper,
            (RPSChoice::Rock, Outcome::Loss) => Self::Scissors,
            (RPSChoice::Scissors, Outcome::Win) => Self::Rock,
            (RPSChoice::Scissors, Outcome::Loss) => Self::Paper,
            (RPSChoice::Paper, Outcome::Win) => Self::Scissors,
            (RPSChoice::Paper, Outcome::Loss) => Self::Rock,
            (choice, Outcome::Draw) => choice,
        }
    }

    fn to_points(&self) -> u32 {
        match self {
            RPSChoice::Rock => 1,
            RPSChoice::Paper => 2,
            RPSChoice::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn to_points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

fn calculate_points(left: RPSChoice, right: RPSChoice) -> [u32; 2] {
    let outcomes: [Outcome; 2] = match (left, right) {
        (RPSChoice::Rock, RPSChoice::Rock) => [Outcome::Draw, Outcome::Draw],
        (RPSChoice::Rock, RPSChoice::Paper) => [Outcome::Loss, Outcome::Win],
        (RPSChoice::Rock, RPSChoice::Scissors) => [Outcome::Win, Outcome::Loss],
        (RPSChoice::Paper, RPSChoice::Rock) => [Outcome::Win, Outcome::Loss],
        (RPSChoice::Paper, RPSChoice::Paper) => [Outcome::Draw, Outcome::Draw],
        (RPSChoice::Paper, RPSChoice::Scissors) => [Outcome::Loss, Outcome::Win],
        (RPSChoice::Scissors, RPSChoice::Rock) => [Outcome::Loss, Outcome::Win],
        (RPSChoice::Scissors, RPSChoice::Paper) => [Outcome::Win, Outcome::Loss],
        (RPSChoice::Scissors, RPSChoice::Scissors) => [Outcome::Draw, Outcome::Draw],
    };

    [
        outcomes[0].to_points() + left.to_points(),
        outcomes[1].to_points() + right.to_points(),
    ]
}

fn day_two_part_one() -> io::Result<u32> {
    let file = File::open("inputs/day2.txt")?;
    let reader = BufReader::new(file);

    let mut opponent_points = 0;
    let mut strategy_points = 0;

    for line in reader.lines().flatten() {
        let opponent_choice = line.chars().nth(0).unwrap();
        let strategy_choice = line.chars().nth(2).unwrap();

        let opponent_rps_choice = match opponent_choice {
            'A' => RPSChoice::Rock,
            'B' => RPSChoice::Paper,
            'C' => RPSChoice::Scissors,
            _ => panic!(),
        };
        let strategy_rps_choice = match strategy_choice {
            'X' => RPSChoice::Rock,
            'Y' => RPSChoice::Paper,
            'Z' => RPSChoice::Scissors,
            _ => panic!(),
        };

        let round_points = calculate_points(opponent_rps_choice, strategy_rps_choice);

        opponent_points += round_points[0];
        strategy_points += round_points[1];
    }

    Ok(strategy_points)
}

fn day_two_part_two() -> io::Result<u32> {
    let file = File::open("inputs/day2.txt")?;
    let reader = BufReader::new(file);

    let mut opponent_points = 0;
    let mut strategy_points = 0;

    for line in reader.lines().flatten() {
        let opponent_choice = line.chars().nth(0).unwrap();
        let strategy_choice = line.chars().nth(2).unwrap();

        let opponent_rps_choice = match opponent_choice {
            'A' => RPSChoice::Rock,
            'B' => RPSChoice::Paper,
            'C' => RPSChoice::Scissors,
            _ => panic!(),
        };
        let strategy_rps_choice = match strategy_choice {
            'X' => RPSChoice::from_intended_outcome(opponent_rps_choice, Outcome::Loss),
            'Y' => RPSChoice::from_intended_outcome(opponent_rps_choice, Outcome::Draw),
            'Z' => RPSChoice::from_intended_outcome(opponent_rps_choice, Outcome::Win),
            _ => panic!(),
        };

        let round_points = calculate_points(opponent_rps_choice, strategy_rps_choice);

        opponent_points += round_points[0];
        strategy_points += round_points[1];
    }

    Ok(strategy_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_points() {
        let round_1_points = calculate_points(RPSChoice::Rock, RPSChoice::Paper)[1];
        let round_2_points = calculate_points(RPSChoice::Paper, RPSChoice::Rock)[1];
        let round_3_points = calculate_points(RPSChoice::Scissors, RPSChoice::Scissors)[1];

        assert_eq!(round_1_points, 8);
        assert_eq!(round_2_points, 1);
        assert_eq!(round_3_points, 6);
    }

    #[test]
    fn test_calculate_day_two_part_one() {
        let result = day_two_part_one().unwrap();

        assert_eq!(result, 10310)
    }
    #[test]
    fn test_calculate_day_two_part_two() {
        let result = day_two_part_two().unwrap();

        assert_eq!(result, 14859)
    }
}
