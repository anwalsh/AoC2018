use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum PointState {
    Empty,
    Taken,
    Overlapping,
}

#[derive(Debug, PartialEq)]
struct Point {
    state: PointState,
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    origin: Point,
    width: u32,
    height: u32,
}

// impl a new function for Claim

fn main() {
    let file = File::open("./data/input.txt").expect("Unable to open file.");
    let buf_read = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    for line in buf_read.lines() {
        let line: String = line.unwrap();
        input.push(line.parse().unwrap());
    }

    // Create graph representing the fabric
    let fabric = vec![vec![0; 1000]; 1000];
}

fn plot_claims() {
    // Takes a Claim as an argument and plots said claim.
}

fn set_claim_state() {}

fn parse_claim(input: String) -> Claim {
    // Input String will look like this: "#1 @ 604,100: 17x27"

    return parsed_claim;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_claim_test() {
        // Test for claim parsing
        // remove and replace with Claim::new() when implemented
        let mut test_claim: Claim = Claim {
            id: 1,
            origin: Point {
                state: PointState::Empty,
                x: 604,
                y: 100,
            },
            width: 17,
            height: 27,
        };

        let test_input: String = "#1 @ 604,100: 17x27".to_string();
        assert_eq!(test_claim, parse_claim(test_input));
    }
}
