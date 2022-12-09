use std::fs;

fn points(play: &str) -> usize{
    match play {
        "A X" => 1+3,
        "A Y" => 2+6,
        "A Z" => 3+0,
        
        "B X" => 1+0,
        "B Y" => 2+3,
        "B Z" => 3+6,
        
        "C X" => 1+6,
        "C Y" => 2+0,
        "C Z" => 3+3,
        _ => 0
    }
}

fn process(file_path: &str) -> usize {
    let contents =
        fs::read_to_string(file_path).expect(format!("missing file {file_path}").as_str());
    contents
        .split("\n")
        .map(|play| points(play))
        .sum::<usize>()
}

fn main() {
    println!("{}", process("./day2/test_data.txt"));
    println!("{}", process("./day2/data.txt"));
}
