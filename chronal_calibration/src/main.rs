use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input: Vec<isize> = Vec::new();
    let file = File::open("./data/input.txt").expect("Unable to open file.");
    let buf_read = BufReader::new(file);
    let mut seen = HashSet::new();
    seen.insert(0);

    for line in buf_read.lines() {
        let line = line.unwrap();
        input.push(line.parse().unwrap());
    }

    let freq = calculate_freq_drift(&input);
    let dup_freq = calculate_first_duplicate_freq(&input);
    println!("Frequency: {} and Duplicate Frequency: {}.", freq, dup_freq);
}

fn calculate_freq_drift(input: &Vec<isize>) -> isize {
    input.iter().sum()
}

fn calculate_first_duplicate_freq(input: &Vec<isize>) -> isize {
    let mut seen_freq = HashSet::new();
    let mut sum: isize = 0;

    input
        .iter()
        .cycle()
        .find_map(|change| {
            sum += change;
            seen_freq.replace(sum)
        }).unwrap()
}
