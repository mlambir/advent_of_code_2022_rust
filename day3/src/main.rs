use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn char_value(c: char) -> usize {
    if c.is_ascii_uppercase() {
        c as usize - 'A' as usize + 27
    } else if c.is_ascii_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        0
    }
}

fn process_line(l: &str) -> usize {
    let splitted = l.split_at(l.len() / 2);
    let s1: HashSet<char> = splitted.0.chars().collect();
    let s2: HashSet<char> = splitted.1.chars().collect();
    s1.intersection(&s2).map(|c| char_value(*c)).sum()
}

fn process(file_path: &str) -> usize {
    let file = File::open(file_path).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| process_line(&l.unwrap()))
        .sum()
}

fn process2(file_path: &str) -> usize {
    let file = File::open(file_path).unwrap();
    let mut total = 0;
    let lines = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>();
    let chunks = lines.chunks(3);
    for sl in chunks {
        let mut chunk_lines = sl.iter().map(|l| l.chars().collect::<HashSet<char>>());
        let mut hs = chunk_lines.next().unwrap();
        for other in chunk_lines {
            hs = hs.intersection(&other).copied().collect();
        }
        total = total + hs.iter().map(|c| char_value(*c)).sum::<usize>()
    }
    total
}

fn main() {
    println!("{}", process("./day3/test_data.txt"));
    println!("{}", process("./day3/data.txt"));

    println!("{}", process2("./day3/test_data.txt"));
    println!("{}", process2("./day3/data.txt"));
}
