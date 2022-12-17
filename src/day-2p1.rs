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
                'a' | 'x' => Ok(Move::Rock),
                'b' | 'y' => Ok(Move::Paper),
                'c' | 'z' => Ok(Move::Scissors),
                _ => Err(ParseMoveError {}),
            },
            None => Err(ParseMoveError {}),
        }
    }
}

struct Game {
    them: Move,
    us: Move,
}

impl Game {
    pub fn score(&self) -> u32 {
        let base = match self.them {
            Move::Rock => match self.us {
                Move::Rock => 3,
                Move::Paper => 6,
                Move::Scissors => 0,
            },
            Move::Paper => match self.us {
                Move::Rock => 0,
                Move::Paper => 3,
                Move::Scissors => 6,
            },
            Move::Scissors => match self.us {
                Move::Rock => 6,
                Move::Paper => 0,
                Move::Scissors => 3,
            },
        };
        base + self.us.value()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (them, us) = s.split_once(' ').ok_or(ParseGameError)?;

        let them_fromstr = them.parse::<Move>().map_err(|_| ParseGameError)?;
        let us_fromstr = us.parse::<Move>().map_err(|_| ParseGameError)?;

        Ok(Game {
            them: them_fromstr,
            us: us_fromstr,
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
