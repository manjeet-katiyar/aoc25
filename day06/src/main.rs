fn part1(lines: &Vec<String>) {
    let n = lines.len();

    let grid: Vec<Vec<i64>> = lines[..n - 1]
        .iter()
        .map(|line| line.split(' ').filter_map(|val| val.parse().ok()).collect())
        .collect();

    let op: Vec<char> = lines[n - 1..]
        .iter()
        .flat_map(|line| line.split(' ').filter_map(|token| token.chars().next()))
        .collect();

    let res: Option<i64> = grid
        .iter()
        .cloned()
        .reduce(|mut acc, e| {
            for ((a, b), o) in acc.iter_mut().zip(e.iter()).zip(op.iter()) {
                if o == &'+' {
                    *a += b;
                } else {
                    *a *= b;
                }
            }
            acc
        })
        .map(|v| v.iter().sum());

    println!("Part1: {}", res.unwrap_or(0));
}

fn part2(lines: &Vec<String>) {
    let n = lines.len();
    let m = lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or_default();

    let op: Vec<char> = lines[n - 1..]
        .iter()
        .flat_map(|line| line.split(' ').filter_map(|token| token.chars().next()))
        .collect();

    let mut temp: Vec<i64> = vec![];
    let mut res: i64 = 0;
    let mut idx: usize = 0;
    for col in 0..m {
        let mut curr: i64 = -1;
        for row in 0..n - 1 {
            let c = lines[row].chars().nth(col).unwrap_or(' ');
            if c == ' ' {
                continue;
            }

            let digit = c.to_digit(10).unwrap() as i64;
            if curr == -1 {
                curr = digit;
            } else {
                curr = curr * 10 + digit;
            }
        }

        if curr == -1 {
            let mut tres: i64 = if op[idx] == '+' { 0 } else { 1 };
            for j in &temp {
                if op[idx] == '+' {
                    tres += j;
                } else {
                    tres *= j;
                }
            }
            idx += 1;
            res += tres;
            temp.clear();
        } else {
            temp.push(curr);
        }
    }

    if !temp.is_empty() {
        let mut tres: i64 = if op[idx] == '+' { 0 } else { 1 };
        for j in temp {
            if op[idx] == '+' {
                tres += j;
            } else {
                tres *= j;
            }
        }
        res += tres;
    }

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day06/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day06/input");
    part1(&lines);
    part2(&lines);
}
