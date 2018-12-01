use std::io::{BufReader, Read};
use std::fs::File;

fn main() {
    let mut payload = String::new();

    let file = File::open("./data/input.txt").expect("Unable to open file.");
    let mut buf_read = BufReader::new(file);

    buf_read.read_to_string(&mut payload).expect("Unable to parse.");
    println!("{}", payload);
}


fn parse_token() {
    
}

fn parse_line() {
    
}

fn calculate_freq_drift() {
    
}
