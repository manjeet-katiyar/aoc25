fn part1(lines: &Vec<String>) {
    let mut itr = lines.split(|line| line.is_empty());
    let mut ranges: Vec<(i64, i64)> = itr
        .next()
        .unwrap()
        .iter()
        .filter_map(|line| match line.split_once('-') {
            Some((a, b)) => Some((a.parse().unwrap(), b.parse().unwrap())),
            None => None,
        })
        .collect();

    ranges.sort();

    let queries: Vec<i64> = itr
        .next()
        .unwrap()
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let merged_ranges = ranges
        .into_iter()
        .fold(Vec::new(), |mut acc, (start, end)| {
            match acc.last_mut() {
                Some((_, last_end)) if start <= *last_end => {
                    *last_end = (*last_end).max(end);
                }
                _ => acc.push((start, end)),
            }
            acc
        });

    let res = queries
        .iter()
        .filter(|&&q| {
            merged_ranges
                .binary_search_by(|&(a, b)| {
                    if q < a {
                        std::cmp::Ordering::Greater
                    } else if q > b {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .count();

    println!("Part1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let mut itr = lines.split(|line| line.is_empty());
    let mut ranges: Vec<(i64, i64)> = itr
        .next()
        .unwrap()
        .iter()
        .filter_map(|line| match line.split_once('-') {
            Some((a, b)) => Some((a.parse().unwrap(), b.parse().unwrap())),
            None => None,
        })
        .collect();

    ranges.sort();

    let res: i64 = ranges
        .into_iter()
        .fold(Vec::new(), |mut acc, (start, end)| {
            match acc.last_mut() {
                Some((_, last_end)) if start <= *last_end => {
                    *last_end = (*last_end).max(end);
                }
                _ => acc.push((start, end)),
            }
            acc
        })
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day05/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day05/input");
    part1(&lines);
    part2(&lines);
}
