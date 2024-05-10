use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn part_one() -> u64 {
    let filepath = "./inputs/day04_part1.txt";
    let file = std::fs::File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|result| result.unwrap())
        .filter(|s| s.len() > 0);
    let mut res = 0;
    for line in lines {
        let (_, line) = line.split_once(": ").unwrap();
        let (mut left, mut right) = line.split_once(" | ").unwrap();
        left = left.trim();
        right = right.trim();

        // println!("{left}---{right}");

        let left: HashSet<_> = left
            .split(" ")
            .filter(|c| c.len() > 0 && c.chars().next().unwrap() != ' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let right: HashSet<_> = right
            .split(" ")
            .filter(|c| c.len() > 0 && c.chars().next().unwrap() != ' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let n = left.intersection(&right).count() as u32;
        if n == 0 {
            continue;
        }
        res += 2u64.pow(n - 1);
    }
    res
}

#[test]
fn test_part1() {
    println!("{}", part_one());
}
