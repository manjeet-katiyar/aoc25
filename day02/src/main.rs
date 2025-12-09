use std::collections::HashSet;

fn part1(lines: &Vec<String>) {
    let ranges: Vec<Vec<i64>> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|r| r.split('-').filter_map(|v| v.parse().ok()).collect())
        .collect();

    let res: i64 = ranges
        .iter()
        .map(|range| {
            let start = range[0];
            let end = range[1];
            let mut temp: i64 = 0;

            for i in 1..100000 {
                let val: i64 = format!("{}{}", i, i).parse().unwrap();
                if val > end {
                    break;
                }

                if val >= start {
                    temp += val;
                }
            }

            temp
        })
        .sum();
    println!("Part1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let ranges: Vec<Vec<i64>> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|r| r.split('-').filter_map(|v| v.parse().ok()).collect())
        .collect();

    let res: i64 = ranges
        .iter()
        .map(|range| {
            let start = range[0];
            let end = range[1];

            let mut st: HashSet<i64> = HashSet::new();

            for i in 1..100000 {
                for k in 2..12 {
                    let val: i64 = i.to_string().repeat(k).parse().unwrap();
                    if val > end {
                        break;
                    }

                    if val >= start {
                        st.insert(val);
                    }
                }
            }

            st.iter().sum::<i64>()
        })
        .sum();
    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day02/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day02/input");
    part1(&lines);
    part2(&lines);
}
