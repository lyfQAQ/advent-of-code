use std::io::BufRead;

fn part_one() -> u32 {
    let filepath = "./inputs/day03.txt";
    let file = std::fs::File::open(filepath).unwrap();
    let mut graph = vec![];
    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map(|result| result.unwrap()) {
        graph.push(line.into_bytes());
    }
    let n = graph.len();
    let m = graph[0].len();
    // println!("{:?}", graph);
    let mut res = 0;
    let mut i: usize = 0;
    while i < n {
        let line = &graph[i];
        let mut start: usize = 0;
        let mut j: usize = 0;
        while !line[j].is_ascii_digit() {
            j += 1;
        }
        // 数字的第一位
        start = j;
        while j < line.len() {
            let mut flag = false;
            while j < line.len() && line[j].is_ascii_digit() {
                if check(&graph, i as i32, j as i32) {
                    flag = true;
                }
                j += 1;
            }
            // 找到了完整数字
            if flag {
                let s = String::from_utf8(line[start..j].to_vec()).unwrap();
                res += s.parse::<u32>().unwrap();
            }
            // 找数字的第一位
            while j < line.len() && !line[j].is_ascii_digit() {
                j += 1;
            }
            start = j;
        }
        i += 1;
    }
    res
}

// 检查每个数字周围是否符合条件
fn check(graph: &Vec<Vec<u8>>, x: i32, y: i32) -> bool {
    // 8 个方向
    let dx = [0, -1, -1, -1, 0, 1, 1, 1];
    let dy = [-1, -1, 0, 1, 1, 1, 0, -1];

    let n = graph.len();
    let m = graph[0].len();
    for k in 0..8 {
        let u: i32 = x + dx[k];
        let v: i32 = y + dy[k];
        if u < 0 || u >= n as i32 || v < 0 || v >= m as i32 {
            continue;
        }
        if graph[u as usize][v as usize] != b'.' && !graph[u as usize][v as usize].is_ascii_digit()
        {
            return true;
        }
    }
    false
}

#[test]
fn test_1() {
    println!("{}", part_one());
}
