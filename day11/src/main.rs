use std::collections::HashMap;

fn dfs_part1(adj: &HashMap<String, Vec<String>>, node: &str) -> i32 {
    if node == "out" {
        return 1;
    }

    let mut res = 0;
    if let Some(neighbours) = adj.get(node) {
        for neighbour in neighbours {
            res += dfs_part1(adj, neighbour);
        }
    }

    return res;
}

fn part1(lines: &Vec<String>) {
    let parsed: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split(" ")
                .enumerate()
                .map(|(idx, w)| {
                    if idx == 0 {
                        w[..w.len() - 1].to_string()
                    } else {
                        w.trim().to_string()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect();

    let adj: HashMap<String, Vec<String>> = parsed
        .iter()
        .map(|row| {
            let key = row[0].clone();
            let value = row[1..].to_vec();
            (key, value)
        })
        .collect();

    let res = dfs_part1(&adj, "you");
    println!("Part1: {}", res);
}

fn dfs_part2(
    adj: &HashMap<String, Vec<String>>,
    mem: &mut HashMap<(String, bool, bool), i64>,
    node: &str,
    is_fft: bool,
    is_dac: bool,
) -> i64 {
    if node == "out" {
        return i64::from(is_fft && is_dac);
    }

    let key = (node.to_string(), is_fft, is_dac);
    if let Some(&val) = mem.get(&key) {
        return val;
    }

    let res = adj.get(node).map_or(0, |neighbours| {
        neighbours
            .iter()
            .map(|neighbour| {
                dfs_part2(
                    adj,
                    mem,
                    neighbour,
                    is_fft || neighbour == "fft",
                    is_dac || neighbour == "dac",
                )
            })
            .sum()
    });

    mem.insert(key, res);
    res
}

fn part2(lines: &Vec<String>) {
    let parsed: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split(" ")
                .enumerate()
                .map(|(idx, w)| {
                    if idx == 0 {
                        w[..w.len() - 1].to_string()
                    } else {
                        w.trim().to_string()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect();

    let adj: HashMap<String, Vec<String>> = parsed
        .iter()
        .map(|row| {
            let key = row[0].clone();
            let value = row[1..].to_vec();
            (key, value)
        })
        .collect();

    let mut mem: HashMap<(String, bool, bool), i64> = HashMap::new();
    let res = dfs_part2(&adj, &mut mem, "svr", false, false);
    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day11/input.test1");
    part1(&lines);

    let lines = utils::read_input("day11/input.test2");
    part2(&lines);

    let lines = utils::read_input("day11/input");
    part1(&lines);
    part2(&lines);
}
