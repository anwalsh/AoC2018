use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./data/input.txt").expect("Unable to open file.");
    let buf_read = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    for line in buf_read.lines() {
        let line: String = line.unwrap();
        input.push(line.parse().unwrap());
    }

    generate_checksum(&input);
}

fn generate_checksum(input: &Vec<String>) {
    for id in input {
        println!("{}", id)
    }
}

fn calculate_checksum(x: i64, y: i64) -> i64 {
    x * y
}
