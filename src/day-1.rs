use std::env;
use std::fs;

fn main() {
    // path to input data
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <path to input file>", args[0]);
        return;
    }

    let file_path = args.get(1).unwrap();

    let contents = fs::read_to_string(file_path).unwrap();
    let entries = contents.split('\n');

    // split the entries into separate collections per elf
    let elves = entries.fold(
        Vec::<Vec<u64>>::from([Vec::<u64>::new()]),
        |mut elves, item| {
            match item.parse::<u64>() {
                Ok(calories) => elves.last_mut().unwrap().push(calories),
                Err(_) => {
                    // found a separator (unparsable line)
                    elves.push(Vec::<u64>::new());
                }
            };
            elves
        },
    );

    // compute the total calories per elf
    let calories_per_elf: Vec<u64> = elves
        .iter()
        .map(|items| items.iter().sum::<u64>())
        .collect();

    // find the elf with the maximum calories
    let top = calories_per_elf.iter().max().unwrap();
    println!("top elf: {}", top);

    // find the elf with the maximum calories
    let mut calories_per_elf_sorted = calories_per_elf.clone();
    calories_per_elf_sorted.sort();

    let total = calories_per_elf_sorted.iter().rev().take(3).sum::<u64>();
    println!("top 3 elves: {}", total);
}
