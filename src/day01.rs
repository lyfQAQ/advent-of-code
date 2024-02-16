use std::fs;
use std::io::{BufRead, BufReader};

fn solve_part_one() -> u32 {
    let filepath = "./inputs/day01.txt";
    let file = fs::File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut res = 0;

    for line in reader.lines().filter_map(|result| result.ok()) {
        let nums = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>();
        let a = *nums.iter().nth(0).unwrap();
        let b = *nums.iter().last().unwrap();
        res += (a as u32) * 10 + b as u32;
    }
    res
}
// one two three four five six seven eight nine
fn solve_part_two() -> u32 {
    let filepath = "./inputs/day01.txt";
    let file = fs::File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let nums = vec![
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    let mut res = 0;
    for line in reader.lines().filter_map(|result| result.ok()) {
        let mut first = None;
        let mut last = None;
        let line = line.as_str();
        'outer: for i in 0..line.len() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if &line[i..i + num.len()] == *num {
                    if first == None {
                        first = Some(1 + index / 2);
                    }
                    last = Some(1 + index / 2);
                }
            }
        }
        let a = first.unwrap() as u32;
        let b = last.unwrap() as u32;
        // println!("{line} -> {first:?}   {last:?}");
        res += a * 10 + b;
    }
    res
}

#[test]
fn test_solve() {
    println!("{}", solve_part_two());
}
