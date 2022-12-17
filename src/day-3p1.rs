// solution to https://adventofcode.com/2022/day/3 (part 1)

use std::collections::HashSet;
use std::env;

use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // path to input data
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <path to input file>", args[0]);
        return;
    }

    let file_path = args.get(1).unwrap();

    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);

    let total: u32 = buf
        .lines()
        .map(|line| {
            let line = line.unwrap_or("".to_string());
            let half_length = line.len() / 2;

            // build sets for each of the two rucksacks
            let mut a = HashSet::new();
            let mut b = HashSet::new();
            line.char_indices().for_each(|(position, character)| {
                if position < half_length {
                    a.insert(character);
                } else {
                    b.insert(character);
                }
            });

            (a, b)
        })
        .map(|(a, b)| *a.intersection(&b).nth(0).unwrap())
        .map(|item| match item {
            'a'..='z' => ((item as u32) - ('a' as u32)) + 1,
            'A'..='Z' => ((item as u32) - ('A' as u32)) + 27,
            _ => 0,
        })
        .sum();

    println!("sum of priorities: {}", total);
}
