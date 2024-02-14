use std::fs;
use std::io::{BufRead, BufReader};

pub fn solve() -> u32 {
    let filename = "./inputs/day01.txt";
    let file = fs::File::open(filename).expect("ERROR: open file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut res = 0;
    loop {
        let n = reader.read_line(&mut line).unwrap();
        if n <= 0 {
            break;
        }
        let data = line.chars().collect::<Vec<char>>();
        let a = data.iter().find(|c| c.is_ascii_digit()).unwrap();
        let b = data.iter().rev().find(|c| c.is_ascii_digit()).unwrap();
        let a = a.to_digit(10).unwrap();
        let b = b.to_digit(10).unwrap();
        res += a * 10 + b;
        line.clear();
    }
    res
}

#[test]
fn test_solve() {
    println!("{}", solve());
}
