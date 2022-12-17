// solution to https://adventofcode.com/2022/day/2 (part 2)

use std::env;
use std::fs;

use std::str::FromStr;

// Enum representing the move in the game
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().chars().nth(0) {
            Some(c) => match c {
                'a' => Ok(Move::Rock),
                'b' => Ok(Move::Paper),
                'c' => Ok(Move::Scissors),
                _ => Err(ParseMoveError {}),
            },
            None => Err(ParseMoveError {}),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn value(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOutcomeError;

impl FromStr for Outcome {
    type Err = ParseOutcomeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().chars().nth(0) {
            Some(c) => match c {
                'x' => Ok(Outcome::Lose),
                'y' => Ok(Outcome::Draw),
                'z' => Ok(Outcome::Win),
                _ => Err(ParseOutcomeError {}),
            },
            None => Err(ParseOutcomeError {}),
        }
    }
}

struct Game {
    them: Move,
    outcome: Outcome,
}

impl Game {
    pub fn score(&self) -> u32 {
        let shape = match self.them {
            Move::Rock => match self.outcome {
                Outcome::Lose => Move::Scissors,
                Outcome::Draw => Move::Rock,
                Outcome::Win => Move::Paper,
            },
            Move::Paper => match self.outcome {
                Outcome::Lose => Move::Rock,
                Outcome::Draw => Move::Paper,
                Outcome::Win => Move::Scissors,
            },
            Move::Scissors => match self.outcome {
                Outcome::Lose => Move::Paper,
                Outcome::Draw => Move::Scissors,
                Outcome::Win => Move::Rock,
            },
        };

        shape.value() + self.outcome.value()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (them, outcome) = s.split_once(' ').ok_or(ParseGameError)?;

        let them_fromstr = them.parse::<Move>().map_err(|_| ParseGameError)?;
        let outcome_fromstr = outcome.parse::<Outcome>().map_err(|_| ParseGameError)?;

        Ok(Game {
            them: them_fromstr,
            outcome: outcome_fromstr,
        })
    }
}

fn main() {
    // path to input data
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <path to input file>", args[0]);
        return;
    }

    let file_path = args.get(1).unwrap();

    let contents = fs::read_to_string(file_path).unwrap();
    let score: u32 = contents
        .split("\n")
        .filter_map(|game_str| match game_str.parse::<Game>() {
            Ok(game) => Some(game),
            Err(_) => None,
        })
        .map(|game| game.score())
        .sum();
    println!("score: {}", score);
}
