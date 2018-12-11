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

    // I think creating a two dimensional vec with 0s for each value initially
    let fabric = vec![vec![0; 1000]; 1000];

    // Claims depicted by 1
    // Overlapping claims depicted by 2
}
