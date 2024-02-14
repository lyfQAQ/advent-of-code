use std::fs;
use std::io::{BufRead, BufReader};

pub fn solve() -> u32 {
    let filename = "./inputs/day01.txt";
    let file = fs::File::open(filename).expect("ERROR: open file");
    let reader = BufReader::new(file);
    let mut res = 0;

    for line in reader.lines().filter_map(|result| result.ok()) {
        let a = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let b = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        let a = a.to_digit(10).unwrap();
        let b = b.to_digit(10).unwrap();
        res += a * 10 + b;
    }
    res
}

#[test]
fn test_solve() {
    println!("{}", solve());
}
