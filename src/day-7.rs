// solution to https://adventofcode.com/2022/day/7

use std::collections::HashMap;
use std::env;
use std::fs;

struct FileEntry {
    name: String,
    size: usize,
}

struct DirEntry {
    name: String,
    subdirs: HashMap<String, DirEntry>,
    files: HashMap<String, FileEntry>,
}

impl DirEntry {
    fn named(name: String) -> DirEntry {
        DirEntry {
            name: name,
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn directory_for_path_mut<'a>(&'a mut self, path: &Vec<String>) -> &'a mut DirEntry {
        let mut cwd = self;
        for p in path {
            cwd = cwd.subdirs.get_mut(p).unwrap();
        }
        cwd
    }

    fn recursive_size(&self) -> usize {
        let local_size: usize = self.files.iter().map(|(_, file)| file.size).sum();
        let remote_size: usize = self
            .subdirs
            .iter()
            .map(|(_, dir)| dir.recursive_size())
            .sum();
        local_size + remote_size
    }

    fn iter(&self) -> DirEntryIter<'_> {
        DirEntryIter::new(&self)
    }
}

struct DirEntryIter<'a> {
    top: &'a DirEntry,
    current: Option<Vec<DirEntryIter<'a>>>,
}

impl<'a> DirEntryIter<'a> {
    fn new(top: &'a DirEntry) -> DirEntryIter<'a> {
        DirEntryIter {
            top: top,
            current: None,
        }
    }
}

// TODO: try to rework this into something a bit more sane
impl<'a> Iterator for DirEntryIter<'a> {
    type Item = &'a DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current {
            Some(child) => {
                if let Some(s) = child.last_mut() {
                    if let Some(t) = s.next() {
                        Some(t)
                    } else {
                        child.pop();
                        self.next()
                    }
                } else {
                    None
                }
            }
            None => {
                self.current = Some(self.top.subdirs.iter().map(|(_, dir)| dir.iter()).collect());
                Some(self.top)
            }
        }
    }
}

enum Command {
    None,
    List,
}

// build the filesystem heirarchy
fn parse_console_output<'a>(contents: String) -> DirEntry {
    let mut root = DirEntry::named("/".to_string());
    let mut path: Vec<String> = Vec::new();
    let mut command = Command::None;
    contents.split("\n").for_each(|entry| {
        let components: Vec<&str> = entry.split_ascii_whitespace().collect();
        if components.len() == 0 {
            return;
        }

        let record = components[0];
        match record {
            "$" => match components[1] {
                "cd" => {
                    let dirname = components[2];
                    match dirname {
                        "/" => {
                            path = Vec::new();
                        }
                        ".." => {
                            path.pop();
                        }
                        _ => {
                            path.push(dirname.to_string());
                        }
                    }
                    command = Command::None;
                }
                "ls" => {
                    command = Command::List;
                }
                _ => {
                    eprintln!("unknown command: {}", components[1]);
                    return;
                }
            },
            "dir" => match command {
                Command::List => {
                    // insert new directory
                    let dirname = components[1].to_string();
                    root.directory_for_path_mut(&path)
                        .subdirs
                        .insert(dirname.clone(), DirEntry::named(dirname.clone()));
                }
                _ => {
                    eprintln!("found record but not in List command");
                    return;
                }
            },
            _ => match command {
                Command::List => {
                    // insert new file
                    let size = components[0].parse::<usize>().unwrap();
                    let filename = components[1].to_string();
                    root.directory_for_path_mut(&path).files.insert(
                        filename.clone(),
                        FileEntry {
                            name: filename.clone(),
                            size: size,
                        },
                    );
                }
                _ => {
                    eprintln!("found record but not in List command");
                    return;
                }
            },
        };
    });
    root
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
    let root = parse_console_output(contents);

    // sum the sizes of all directories of no more than 100,000 in size
    let size: usize = root
        .iter()
        .map(|dir| dir.recursive_size())
        .filter(|size| *size <= 100000)
        .sum();
    println!(
        "total size of directories of no more than 100,000 in size: {}",
        size
    );

    // total size of the filesystem
    let disk_size = 70000000usize;
    let usage = root.recursive_size();
    let available = disk_size - usage;
    println!(
        "total size of filesystem: {} (available: {})",
        usage, available
    );

    // find the smallest directory we can delete that makes enough available space
    let update = 30000000usize;
    let requirement = update.checked_sub(available).unwrap_or(0);
    let smallest = root
        .iter()
        .map(|dir| dir.recursive_size())
        .filter(|size| *size >= requirement)
        .min()
        .unwrap();
    println!(
        "smallest directory we can delete: {} (to free: {})",
        smallest, requirement
    );
}
