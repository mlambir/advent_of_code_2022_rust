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

fn main() {
    println!("{}", process("./day1/calories_test.txt"));
    println!("{}", process("./day1/calories.txt"));
}
