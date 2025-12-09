fn part1(lines: &Vec<String>) {
    let n = lines.len();
    let mut res = 0;

    let mut row = vec![0; n];
    for line in lines {
        let mut row_t = row.clone();
        for (idx, c) in line.chars().enumerate() {
            if c == 'S' {
                row_t[idx] = 1;
                break;
            }

            if c == '^' && row[idx] == 1 {
                res += 1;
                row_t[idx] = 0;
                if idx > 0 {
                    row_t[idx - 1] = 1;
                }

                if idx + 1 < n {
                    row_t[idx + 1] = 1;
                }
            }
        }

        row = row_t;
    }

    println!("Part1: {}", res);
}

fn part2_solve(
    lines: &Vec<String>,
    n: usize,
    row: usize,
    col: usize,
    mem: &mut Vec<Vec<i64>>,
) -> i64 {
    if row >= n {
        return 1;
    }

    if mem[row][col] != -1 {
        return mem[row][col];
    }

    let mut res: i64 = 0;
    if let Some(c) = lines[row].chars().nth(col) {
        if c == '^' {
            res += if col > 0 {
                part2_solve(&lines, n, row + 1, col - 1, mem)
            } else {
                0
            };

            res += if col < n - 1 {
                part2_solve(&lines, n, row + 1, col + 1, mem)
            } else {
                0
            };
        } else {
            res = part2_solve(&lines, n, row + 1, col, mem);
        }
    }

    mem[row][col] = res;
    return res;
}

fn part2(lines: &Vec<String>) {
    let n = lines.len();
    let mut res = 0;
    let mut mem = vec![vec![-1; lines[0].len()]; n];
    for (idx, c) in lines[0].chars().enumerate() {
        if c == 'S' {
            res = part2_solve(&lines, n, 1, idx, &mut mem);
        }
    }

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day07/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day07/input");
    part1(&lines);
    part2(&lines);
}
