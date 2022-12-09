use std::arch::x86_64::_mm256_and_pd;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{Range, };

fn to_range(s: &str) -> Range<usize>{
    let splitted:Vec<_> = s.split("-").collect();
    splitted[0].parse::<usize>().unwrap()..splitted[1].parse::<usize>().unwrap()
}

fn filter_line(l: &str) -> bool {
    let splitted:Vec<_> = l.split(",").collect();
    let r1 = to_range(splitted[0]);
    let r2 = to_range(splitted[1]);
    (r1.start <= r2.start && r1.end >= r2.end) || (r2.start <= r1.start && r2.end >= r1.end)
}

fn process(file_path: &str) -> usize {
    let file = File::open(file_path).unwrap();
    io::BufReader::new(file)
        .lines().filter(|l|filter_line(l.as_ref().unwrap())).count()
}

fn main() {
    println!("{}", process("./day4/test_data.txt"));
    println!("{}", process("./day4/data.txt"));
}
