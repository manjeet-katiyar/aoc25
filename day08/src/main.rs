use std::{cmp::Reverse, collections::BinaryHeap};

use utils::dsu::DSU;

fn part1(lines: &Vec<String>, iterations: usize) {
    let n = lines.len();

    let parse = |point: &str| {
        let [x, y, z]: [i64; 3] = point
            .split(',')
            .map(|val| val.parse().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();

        (x, y, z)
    };

    let calculate_dist = |point1: &str, point2: &str| {
        let (x1, y1, z1) = parse(point1);
        let (x2, y2, z2) = parse(point2);

        let dx = x1 - x2;
        let dy = y1 - y2;
        let dz = z1 - z2;

        dx * dx + dy * dy + dz * dz
    };

    let mut pq = BinaryHeap::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist = calculate_dist(&lines[i], &lines[j]);
            pq.push(Reverse((dist, i, j)));
        }
    }

    let mut dsu = DSU::new(n);
    let mut counter = 0;
    while let Some(&Reverse((_dist, i, j))) = pq.peek() {
        pq.pop();
        dsu.merge(i, j);
        counter += 1;
        if counter == iterations {
            break;
        }
    }

    let mut visited = vec![false; n];
    let mut sizes = vec![];
    for i in 0..n {
        let parent = dsu.get_parent(i);
        let size = dsu.get_size(parent);
        if visited[parent] == false {
            sizes.push(size);
            visited[parent] = true;
        }
    }

    sizes.sort();
    let m = sizes.len();
    let mut res = 1;
    for i in 1..=3 {
        if m >= i {
            res *= sizes[m - i];
        }
    }
    println!("Part1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let n = lines.len();

    let parse = |point: &str| {
        let [x, y, z]: [i64; 3] = point
            .split(',')
            .map(|val| val.parse().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();

        (x, y, z)
    };

    let calculate_dist = |point1: &str, point2: &str| {
        let (x1, y1, z1) = parse(point1);
        let (x2, y2, z2) = parse(point2);

        let dx = x1 - x2;
        let dy = y1 - y2;
        let dz = z1 - z2;

        dx * dx + dy * dy + dz * dz
    };

    let mut pq = BinaryHeap::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist = calculate_dist(&lines[i], &lines[j]);
            pq.push(Reverse((dist, i, j)));
        }
    }

    let mut dsu = DSU::new(n);
    let mut res = 0;
    while let Some(&Reverse((_dist, i, j))) = pq.peek() {
        pq.pop();
        if dsu.merge(i, j) {
            let (x1, _y1, _z1) = parse(&lines[i]);
            let (x2, _y2, _z2) = parse(&lines[j]);

            res = x1 * x2;
        }
    }

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day08/input.test");
    part1(&lines, 10);
    part2(&lines);

    let lines = utils::read_input("day08/input");
    part1(&lines, 1000);
    part2(&lines);
}
