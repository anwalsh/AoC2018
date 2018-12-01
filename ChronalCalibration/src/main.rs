use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut payload = String::new();

    let file = File::open("./data/input.txt").expect("Unable to open file.");
    let mut buf_read = BufReader::new(file);

    for line in buf_read.lines() {
        let line = line.expect("Unable to read line.");
        calculate_freq_drift(line);
    }
    // buf_read.read_to_string(&mut payload).expect("Unable to parse.");
    // println!("{}", payload);
}


fn parse_token(l : &str) {
    
}

fn parse_line(l : &str) {
    
}

fn calculate_freq_drift(line : String) {
    let mut freq = 0;
    let len = line.len();
    let slice_line: &str = &line[..];
    let token = &slice_line[..1];
    let drift = &slice_line[1..len];

    println!("The token is {} and the value is {}", token, drift);
}
