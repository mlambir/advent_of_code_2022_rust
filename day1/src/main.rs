use std::fs;

fn process(file_path: &str) -> usize {
    let contents =
        fs::read_to_string(file_path).expect(format!("missing file {file_path}").as_str());
    contents
        .split("\n\n")
        .map(|s| s.split("\n").map(|s| s.parse::<usize>().unwrap_or(0)))
        .map(|v| v.sum::<usize>())
        .max()
        .unwrap()
}

fn process2(file_path: &str) -> usize {
    let contents =
        fs::read_to_string(file_path).expect(format!("missing file {file_path}").as_str());
    let mut elves: Vec<usize> = contents
        .split("\n\n")
        .map(|s| s.split("\n").map(|s| s.parse::<usize>().unwrap_or(0)))
        .map(|v| v.sum::<usize>())
        .collect();
    elves.sort();
    elves.iter().rev().take(3).sum()
}

fn main() {
    println!("{}", process("./day1/calories_test.txt"));
    println!("{}", process("./day1/calories.txt"));

    println!("{}", process2("./day1/calories_test.txt"));
    println!("{}", process2("./day1/calories.txt"));
}
