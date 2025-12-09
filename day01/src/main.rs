fn part1(lines: &Vec<String>) {
    let list: Vec<i32> = lines
        .iter()
        .map(|item| match item {
            c if c.starts_with('R') => c[1..].parse::<i32>().unwrap(),
            c => -1 * c[1..].parse::<i32>().unwrap(),
        })
        .collect();

    let mut start = 50;
    let mut res = 0;
    for i in &list {
        start += i;
        start %= 100;
        if start == 0 {
            res += 1;
        }
    }

    println!("Part1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let list: Vec<i32> = lines
        .iter()
        .map(|item| match item {
            c if c.starts_with('R') => c[1..].parse::<i32>().unwrap(),
            c => -1 * c[1..].parse::<i32>().unwrap(),
        })
        .collect();
    let mut start = 50;
    let mut res = 0;
    for i in &list {
        res += i.abs() / 100;
        let rem = i % 100;

        if start > 0 && start + rem <= 0 {
            res += 1;
        } else if start + rem >= 100 {
            res += 1;
        }

        start += i % 100;
        start += 100;
        start %= 100;
    }

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day01/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day01/input");
    part1(&lines);
    part2(&lines);
}
