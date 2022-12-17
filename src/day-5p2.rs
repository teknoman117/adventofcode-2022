// solution to https://adventofcode.com/2022/day/5 (part 2)
#![feature(iter_next_chunk)]

use std::collections::VecDeque;

use std::env;

use std::fs::File;

use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    // path to input data
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <path to input file>", args[0]);
        return Err(io::Error::new(io::ErrorKind::Other, "invalid arguments"));
    }

    let file_path = args.get(1).unwrap();

    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);

    // build stacks from initial layout
    let mut stacks = Vec::<VecDeque<char>>::new();
    let mut lines = buf.lines();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        // split stack into per-stack items
        let mut line = line.chars();
        let mut index = 0;
        while let Ok(item) = line.next_chunk::<3>() {
            if index >= stacks.len() {
                stacks.push(VecDeque::<char>::new());
            }

            let item = String::from_iter(item);
            if let Some(item) = item
                .strip_prefix("[")
                .and_then(|item| item.strip_suffix("]"))
            {
                stacks[index].push_front(item.chars().nth(0).unwrap());
            }

            // strip trailing whitespace character
            _ = line.next();
            index = index + 1;
        }
    }

    // remaining lines are commands
    for line in lines {
        let line = line?;
        let components: Vec<usize> = line
            .split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .flat_map(|c| c.parse::<usize>())
            .collect();
        let count = components[0];
        let from = components[1] - 1;
        let to = components[2] - 1;
        let index = stacks[from].len() - count;
        let items = stacks[from].split_off(index);
        stacks[to].extend(items);
    }

    let tops: String = stacks
        .iter()
        .filter_map(|stack| stack.back())
        .map(|item| *item)
        .collect();
    println!("tops: {}", tops);

    Ok(())
}
