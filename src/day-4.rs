// solution to https://adventofcode.com/2022/day/4

use std::env;
use std::fs;

use std::str::FromStr;

struct Range {
    start: u32,
    end: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseRangeError)?;

        let start_fromstr = start.parse::<u32>().map_err(|_| ParseRangeError)?;
        let end_fromstr = end.parse::<u32>().map_err(|_| ParseRangeError)?;

        Ok(Range {
            start: start_fromstr,
            end: end_fromstr,
        })
    }
}

struct Assignment(Range, Range);

#[derive(Debug, PartialEq, Eq)]
struct ParseAssignmentError;

impl FromStr for Assignment {
    type Err = ParseAssignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').ok_or(ParseAssignmentError)?;

        let a_fromstr = a.parse::<Range>().map_err(|_| ParseAssignmentError)?;
        let b_fromstr = b.parse::<Range>().map_err(|_| ParseAssignmentError)?;

        Ok(Assignment(a_fromstr, b_fromstr))
    }
}

impl Assignment {
    pub fn has_full_overlap(&self) -> bool {
        (self.0.start >= self.1.start && self.0.end <= self.1.end)
            || (self.1.start >= self.0.start && self.1.end <= self.0.end)
    }

    pub fn has_overlap(&self) -> bool {
        (self.0.start <= self.1.start && self.0.end >= self.1.start)
            || (self.1.start <= self.0.start && self.1.end >= self.0.start)
            || self.has_full_overlap()
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
    let full_overlaps = contents
        .split('\n')
        .flat_map(|line| line.parse::<Assignment>())
        .filter(|a| a.has_full_overlap())
        .count();
    println!("assignments with full overlap: {}", full_overlaps);

    let partial_overlaps = contents
        .split('\n')
        .flat_map(|line| line.parse::<Assignment>())
        .filter(|a| a.has_overlap())
        .count();
    println!("assignments with partial overlap: {}", partial_overlaps);
}
