use utils::read_input;

fn main() {
    let lines = read_input("day01/input");
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
        if start < 0 {
            start += 100;
        }

        start %= 100;
        if start == 0 {
            res += 1;
        }
    }

    dbg!(&res);

    let mut start = 50;
    let mut res = 0;
    for i in &list {
        if start > 0 && start + i <= 0 {
            res += 1;
            res += (start - i) / 100;
        } else if start < 100 && start + i >= 100 {
            res += 1;
            res += (i - (100 - start)) / 100;
        }

        start += i;
        if start < 0 {
            start += 100;
        }

        start %= 100;
    }

    dbg!(&res);
}
