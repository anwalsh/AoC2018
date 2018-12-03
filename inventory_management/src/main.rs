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

    let checksum = generate_checksum(&input);
    println!("Checksum is: {}", checksum);
}

fn generate_checksum(input: &Vec<String>) -> i64 {
    let mut two = 0;
    let mut three = 0;

    for id in input {
        let id_vec: Vec<char> = id.chars().collect();
        let mut _two = 0;
        let mut _three = 0;

        for character in id_vec.iter() {
            let occurrences = id_vec.iter().filter(|c| c == &character).count();

            match occurrences {
                2 => _two += 1,
                3 => _three += 1,
                _ => (),
            }
        }
        if _two >= 1 && _three >= 1 {
            two += 1;
            three += 1;
        } else if _two >= 1 {
            two += 1;
        } else if _three >= 1 {
            three += 1;
        }
    }
    calculate_checksum(two, three)
}

fn calculate_checksum(x: i64, y: i64) -> i64 {
    x * y
}
