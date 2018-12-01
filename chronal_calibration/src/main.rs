use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut freq : isize = 0;
    let file = File::open("./data/input.txt").expect("Unable to open file.");
    let buf_read = BufReader::new(file);

    for line in buf_read.lines() {
        let line = line.expect("Unable to read line.");
        calculate_freq_drift(line, &mut freq);
    }
    println!("The frequency is: {}.", freq);
}

fn parse_line(line : String) -> (isize) {
    let drift : isize = line.parse().expect("Failed to parse isize from line.");

    return drift;
}

fn calculate_freq_drift(line : String, freq : &mut isize) {
    let drift = parse_line(line);
    *freq += drift;
}
