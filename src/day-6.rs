// solution to https://adventofcode.com/2022/day/6

use std::collections::HashSet;
use std::env;
use std::fs;

fn find_non_repeating(contents: &String, group_size: usize) -> usize {
    // slide window over the packet buffer to find 1st group of N non-repeating characters
    (group_size..=contents.len())
        .filter(|head| {
            let window = &contents[*head - group_size..*head];
            let set: HashSet<char> = window.chars().collect();
            set.len() == group_size
        })
        .nth(0)
        .unwrap()
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

    let first_packet = find_non_repeating(&contents, 4);
    println!("first packet marker: {}", first_packet);

    let first_message = find_non_repeating(&contents, 14);
    println!("first message marker: {}", first_message);
}
