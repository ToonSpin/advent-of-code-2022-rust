use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn game_from(s: &&str) -> Game {
        let r = match s.chars().nth(0).unwrap() {
            'A' => Rps::Rock,
            'B' => Rps::Paper,
            'C' => Rps::Scissors,
            _ => unreachable!(),
        };
        let s = match s.chars().nth(2).unwrap() {
            'X' => Rps::Rock,
            'Y' => Rps::Paper,
            'Z' => Rps::Scissors,
            _ => unreachable!(),
        };
        (r, s)
    }

    fn game_from_part2(s: &&str) -> Game {
        let r = match s.chars().nth(0).unwrap() {
            'A' => Rps::Rock,
            'B' => Rps::Paper,
            'C' => Rps::Scissors,
            _ => unreachable!(),
        };
        let desired_outcome = match s.chars().nth(2).unwrap() {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        };
        let s = r.get_move_for_outcome(&desired_outcome);
        (r, s)
    }

    fn get_move_for_outcome(&self, outcome: &Outcome) -> Rps {
        match self {
            Rps::Rock => match outcome {
                Outcome::Win => Rps::Paper,
                Outcome::Loss => Rps::Scissors,
                Outcome::Draw => Rps::Rock,
            },
            Rps::Paper => match outcome {
                Outcome::Win => Rps::Scissors,
                Outcome::Loss => Rps::Rock,
                Outcome::Draw => Rps::Paper,
            },
            Rps::Scissors => match outcome {
                Outcome::Win => Rps::Rock,
                Outcome::Loss => Rps::Paper,
                Outcome::Draw => Rps::Scissors,
            },
        }
    }

    fn outcome(&self, other: &Self) -> Outcome {
        match self {
            Rps::Rock => match other {
                Rps::Rock => Outcome::Draw,
                Rps::Paper => Outcome::Loss,
                Rps::Scissors => Outcome::Win,
            },
            Rps::Paper => match other {
                Rps::Rock => Outcome::Win,
                Rps::Paper => Outcome::Draw,
                Rps::Scissors => Outcome::Loss,
            },
            Rps::Scissors => match other {
                Rps::Rock => Outcome::Loss,
                Rps::Paper => Outcome::Win,
                Rps::Scissors => Outcome::Draw,
            },
        }
    }

    fn score(&self, other: &Self) -> u64 {
        let score = match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };
        score
            + match self.outcome(&other) {
                Outcome::Win => 6,
                Outcome::Loss => 0,
                Outcome::Draw => 3,
            }
    }
}

type Game = (Rps, Rps);

fn get_total_score(games: &Vec<Game>) -> u64 {
    games.iter().map(|(a, b)| b.score(&a)).sum::<u64>()
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.split('\n').filter(|s| s.len() > 0).collect();

    let input_part1 = input.iter().map(Rps::game_from).collect();
    let score_part_1 = get_total_score(&input_part1);
    println!("The score according to your guide: {}", score_part_1);
    
    let input_part2 = input.iter().map(Rps::game_from_part2).collect();
    let score_part_2 = get_total_score(&input_part2);
    println!("The score according to their guide: {}", score_part_2);

    Ok(())
}
