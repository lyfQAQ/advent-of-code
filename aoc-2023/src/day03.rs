use std::hash::Hash;
use std::io::BufRead;

// fn part_one() -> u32 {
//     let filepath = "./inputs/day03.txt";
//     let file = std::fs::File::open(filepath).unwrap();
//     let mut graph = vec![];
//     let reader = std::io::BufReader::new(file);
//     for line in reader.lines().map(|result| result.unwrap()) {
//         graph.push(line.into_bytes());
//     }
//     let n = graph.len();
//     let m = graph[0].len();
//     // println!("{:?}", graph);
//     let mut res = 0;
//     let mut i: usize = 0;
//     while i < n {
//         let line = &graph[i];
//         let mut start: usize = 0;
//         let mut j: usize = 0;
//         while !line[j].is_ascii_digit() {
//             j += 1;
//         }
//         // 数字的第一位
//         start = j;
//         while j < line.len() {
//             let mut flag = false;
//             while j < line.len() && line[j].is_ascii_digit() {
//                 if check_part_one(&graph, i as i32, j as i32) {
//                     flag = true;
//                 }
//                 j += 1;
//             }
//             // 找到了完整数字
//             if flag {
//                 let s = String::from_utf8(line[start..j].to_vec()).unwrap();
//                 res += s.parse::<u32>().unwrap();
//             }
//             // 找数字的第一位
//             while j < line.len() && !line[j].is_ascii_digit() {
//                 j += 1;
//             }
//             start = j;
//         }
//         i += 1;
//     }
//     res
// }

// // 检查每个数字周围是否符合条件
// fn check_part_one(graph: &Vec<Vec<u8>>, x: i32, y: i32) -> bool {
//     // 8 个方向
//     let dx = [0, -1, -1, -1, 0, 1, 1, 1];
//     let dy = [-1, -1, 0, 1, 1, 1, 0, -1];

//     let n = graph.len();
//     let m = graph[0].len();
//     for k in 0..8 {
//         let u: i32 = x + dx[k];
//         let v: i32 = y + dy[k];
//         if u < 0 || u >= n as i32 || v < 0 || v >= m as i32 {
//             continue;
//         }
//         if graph[u as usize][v as usize] != b'.' && !graph[u as usize][v as usize].is_ascii_digit()
//         {
//             return true;
//         }
//     }
//     false
// }

use std::collections::HashSet;
#[derive(Debug)]
struct ParNumber {
    value: i64,
    // 该数字能够扩展的点集
    points: HashSet<(i64, i64)>,
}
impl ParNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        // 当前位相邻的8个方向
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), // 左边
            (row - 1, col),
            (row + 1, col), // 上和下
            (row + 1, col + 1),
            (row, col + 1),
            (row - 1, col + 1), // 右边
        ]);
        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }
    // 添加后续数字位
    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        // 此时只需扩展右边的方向
        self.points
            .extend([(row + 1, col + 1), (row, col + 1), (row - 1, col + 1)]);
    }
}
fn part_one() -> i64 {
    let filepath = "./inputs/day03_part2.txt";
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines().map(|result| result.unwrap());
    let mut nums = vec![];
    // 所有符号的位置
    let mut syms = HashSet::new();

    // part_two needed
    let mut gears = HashSet::new();
    // end

    // 当前正在解析的数字序列
    let mut cur_number: Option<ParNumber> = None;
    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_number {
                    // 继续构造当前数字
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    cur_number = Some(ParNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    nums.push(num); // 当前数字解析完毕
                }
                if ch != '.' {
                    // 记录符号的位置
                    syms.insert((row as i64, col as i64));

                    // part_two add
                    if ch == '*' {
                        gears.insert((row as i64, col as i64));
                    }
                    // end
                }
            }
        }
    }

    // part_one add
    // let total = nums
    //     .iter()
    //     .filter(|num| num.points.intersection(&syms).next().is_some())
    //     .map(|num| num.value)
    //     .sum();
    // end

    // part_two add
    let mut total = 0;
    'next_gear: for gear in &gears {
        let mut matches = vec![];
        // 找相邻 2 个数
        for num in &nums {
            if num.points.contains(gear) {
                if matches.len() == 2 {
                    // 即将超过 2 个 舍去
                    continue 'next_gear;
                }
                matches.push(num.value);
            }
        }
        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }
    // end

    total
}

#[test]
fn test_1() {
    println!("{}", part_one());
}
