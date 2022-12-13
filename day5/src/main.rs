use std::{fs, iter::Enumerate};

use regex::Regex;

fn p(v: &Vec<Vec<char>>) {
    for (n, v) in v.iter().enumerate() {
        println!("{n} - {:?}", v)
    }
}
fn parse_crates(crates_txt: &str) -> Vec<Vec<char>> {
    let mut crates = crates_txt.rsplit("\n");
    let col_nums = crates.next().unwrap();
    let col_count = col_nums
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .count();
    let re = Regex::new(r"\[[A-Z]\]\s?|(\s{3,4})").unwrap();
    let mut v: Vec<Vec<char>> = vec![];
    for _ in 0..col_count {
        v.push(vec![]);
    }
    for c in crates {
        for (col, found) in re.find_iter(c).enumerate() {
            match found.as_str().chars().nth(1) {
                None => (),
                Some(' ') => (),
                Some(c) => v[col].push(c),
            }
        }
    }
    v
}

fn parse_moves(moves_txt: &str) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    moves_txt
        .split("\n")
        .into_iter()
        .map(|l| {
            let m = re.captures(l).unwrap();
            (
                m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                m.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            )
        })
        .collect()
}

fn process(file_path: &str) -> String {
    let contents =
        fs::read_to_string(file_path).expect(format!("missing file {file_path}").as_str());
    let (crates, moves) = contents.split_once("\n\n").unwrap();

    let mut v = parse_crates(crates);
    for (count, from, to) in parse_moves(moves) {
        for _ in 0..count {
            let c = v[from].pop().unwrap();
            v[to].push(c);
        }
    }

    v.iter().map(|v1| v1.last().unwrap()).collect()
}

fn main() {
    println!("{}", process("./day5/test_data.txt"));
    println!("{}", process("./day5/data.txt"));
}
