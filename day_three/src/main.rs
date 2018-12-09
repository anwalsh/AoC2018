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
}
