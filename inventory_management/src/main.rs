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
    let mut x = 0;
    let mut y = 0;

    for id in input {
        let id_vec: Vec<char> = id.chars().collect();
        let mut seen_twice = 0;
        let mut seen_thrice = 0;

        for character in id_vec.iter() {
            let occurrences = id_vec.iter().filter(|c| c == &character).count();

            if occurrences > 3 {
                println!("{}", occurrences);
            }
            match occurrences {
                2 => seen_twice += 1,
                3 => seen_thrice += 1,
                _ => (),
            }
            // println!("{}, {}", seen_twice, seen_thrice);
        }
        if seen_twice >= 1 && seen_thrice >= 1 {
            x += 1;
            y += 1;
        }
        else if seen_twice >= 1 {
            x += 1;
        }
        else if seen_thrice >= 1 {
            y += 1;
        }
    }
    calculate_checksum(x, y)
}

fn calculate_checksum(x: i64, y: i64) -> i64 {
    x * y
}
