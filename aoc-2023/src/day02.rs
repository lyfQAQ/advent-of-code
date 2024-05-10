use std::io::BufRead;

fn solve_part_one() -> u32 {
    let mut hash = std::collections::HashMap::new();
    let mut res = 0_u32;
    hash.insert("red", 12);
    hash.insert("green", 13);
    hash.insert("blue", 14);
    //Game 1: 7 green, 4 blue, 3 red; 4 blue, 10 red, 1 green; 1 blue, 9 red
    let filepath = "./inputs/day02.txt";
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    for (i, line) in reader.lines().filter_map(|result| result.ok()).enumerate() {
        let (_, turns) = line.split_once(": ").unwrap();
        let turns = turns.split("; ").collect::<Vec<_>>();
        let mut flag = true;
        'outer: for turn in turns {
            for t in turn.split(", ") {
                let (n, color) = t.split_once(" ").unwrap();
                let n = n.parse::<u32>().unwrap();
                if n > hash[color] {
                    flag = false;
                    break 'outer;
                }
            }
        }
        if (flag) {
            res += i as u32 + 1;
        }
    }
    res
}

fn solve_part_two() -> u32 {
    let filepath = "./inputs/day02.txt";
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut res = 0;
    //Game 1: 7 green, 4 blue, 3 red; 4 blue, 10 red, 1 green; 1 blue, 9 red
    for line in reader.lines().filter_map(|result| result.ok()) {
        let (_, turns) = line.split_once(": ").unwrap();
        let turns = turns.split("; ").collect::<Vec<_>>();

        // 每小组找对应颜色最多的
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for turn in turns {
            for t in turn.split(", ") {
                let (n, color) = t.split_once(" ").unwrap();
                let n = n.parse::<u32>().unwrap();
                match color {
                    "red" => red = std::cmp::max(red, n),
                    "green" => green = std::cmp::max(green, n),
                    _ => blue = std::cmp::max(blue, n),
                }
            }
        }
        res += red * green * blue;
    }
    res
}

#[test]
fn test_solve() {
    println!("{}", solve_part_two());
}
