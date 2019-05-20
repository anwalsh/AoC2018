use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
    x: u32,
    y: u32,
}

enum ClaimState {
    Empty,
    Taken,
    Overlapping,
}

struct Claim {
    width: u32,
    height: u32,
}

// TODO: Implement a encapsulating data structure which holds Claim_Id, Point, ClaimState, and Claim

fn main() {
    let file = File::open("./data/input.txt").expect("Unable to open file.");
    let buf_read = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    for line in buf_read.lines() {
        let line: String = line.unwrap();
        input.push(line.parse().unwrap());
    }

    let fabric = vec![vec![0; 1000]; 1000];

    // for (_y, _row) in fabric.iter().enumerate() {
    //     println!("{:?}", _row);
    // }
}

fn plot_claims(x: i32, y: i32, claim_x: i32, claim_y: i32) {
    // Input appears as below
    // #1 @ 604,100: 17x27
    // 604 from left edge, 100 from the top edge, and 17x27 wide
    // #2 @ 861,26: 23x24
    // 861 from the left, 26 from the top, and 23x24 wide.
}

fn parse_line(input: String) {
    // Ignore the claim number and @ symbol. For the plot pass the remaining string as variables
    // to plot claims where the underlying plot will be changed to the following:
    // - 0 (default) unclaimed fabric
    // - 1 claimed fabric
    // - 2 overlapping fabric
}

// TODO: Implement tests for two to four line parses and appropriately populating the data structure
