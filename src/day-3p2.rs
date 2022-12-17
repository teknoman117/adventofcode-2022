// solution to https://adventofcode.com/2022/day/3 (part 2)

#![feature(iter_next_chunk)]

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

    let mut rucksacks = buf
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
        .map(|(a, b)| -> HashSet<char> {
            // is there any way to avoid making a concrete set here?
            a.union(&b).map(|c| *c).collect()
        });

    // iterate over 3-groups of rucksack set-unions (this impl doesn't seem like it'd be fast)
    let mut total: u32 = 0;
    while let Ok([a, b, c]) = rucksacks.next_chunk::<3>() {
        // again, is there any way to avoid concretely flattening the set?
        let d: HashSet<char> = a.intersection(&b).map(|c| *c).collect();
        let priority: u32 = d
            .intersection(&c)
            .map(|c| *c)
            .map(|item| match item {
                'a'..='z' => ((item as u32) - ('a' as u32)) + 1,
                'A'..='Z' => ((item as u32) - ('A' as u32)) + 27,
                _ => 0,
            })
            .sum();

        total += priority;
    }
    println!("sum of priorities: {}", total);
}
