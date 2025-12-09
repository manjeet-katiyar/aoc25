use std::{
    cmp::{max, min},
    collections::{BTreeSet, HashMap, VecDeque},
};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(lines: &Vec<String>) {
    let n = lines.len();
    let parsed = lines
        .iter()
        .map(|line| {
            let [y, x]: [i64; 2] = line
                .split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap();
            (x, y)
        })
        .collect::<Vec<(i64, i64)>>();

    let res = parsed
        .iter()
        .enumerate()
        .flat_map(|(i, &(x1, y1))| {
            parsed[i + 1..n]
                .iter()
                .map(move |(x2, y2)| ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1))
                .collect::<Vec<i64>>()
        })
        .max()
        .unwrap_or_default();

    println!("Part1: {}", res);
}

fn flood(grid: &mut Vec<Vec<i64>>, grid_size: usize, x: usize, y: usize) {
    if grid[x][y] != 0 {
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    grid[x][y] = 2;
    while let Some(&(xc, yc)) = queue.front() {
        queue.pop_front();

        grid[xc][yc] = 2;
        DIRECTIONS
            .iter()
            .flat_map(|&(dx, dy)| {
                let xn = xc.checked_add_signed(dx);
                let yn = yc.checked_add_signed(dy);

                match (xn, yn) {
                    (Some(xn), Some(yn)) if xn < grid_size && yn < grid_size => Some((xn, yn)),
                    _ => None,
                }
            })
            .collect::<Vec<(usize, usize)>>()
            .iter()
            .for_each(|&(xn, yn)| {
                if grid[xn][yn] == 0 {
                    grid[xn][yn] = 2;
                    queue.push_back((xn, yn));
                }
            });
    }
}

fn add_boundary(grid: &mut Vec<Vec<i64>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 == x2 {
        for i in min(y1, y2)..=max(y1, y2) {
            grid[x1][i] = 1;
        }
    } else if y1 == y2 {
        for i in min(x1, x2)..=max(x1, x2) {
            grid[i][y1] = 1;
        }
    }
}

fn part2(lines: &Vec<String>) {
    let n = lines.len();
    let parsed = lines
        .iter()
        .map(|line| {
            let [y, x]: [i64; 2] = line
                .split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap();
            vec![x, y]
        })
        .collect::<Vec<Vec<i64>>>();

    let unique_points = BTreeSet::from_iter(parsed.iter().flatten().copied());
    let compressed_c: HashMap<i64, usize> = HashMap::from_iter(
        unique_points
            .iter()
            .enumerate()
            .map(|(idx, &val)| (val, idx)),
    );

    let grid_size = compressed_c.len() + 1;
    let mut grid: Vec<Vec<i64>> = vec![vec![0; grid_size]; grid_size];

    for (i, point) in parsed.iter().enumerate() {
        let [x, y]: [usize; 2] = point
            .iter()
            .map(|k| compressed_c.get(k).unwrap().clone())
            .collect::<Vec<usize>>()
            .as_slice()
            .try_into()
            .unwrap();

        let pre = if i == 0 { n - 1 } else { i - 1 };
        let nxt = if i == n - 1 { 0 } else { i + 1 };

        let [xp, yp]: [usize; 2] = parsed[pre]
            .iter()
            .map(|k| compressed_c.get(k).unwrap().clone())
            .collect::<Vec<usize>>()
            .as_slice()
            .try_into()
            .unwrap();
        let [xn, yn]: [usize; 2] = parsed[nxt]
            .iter()
            .map(|k| compressed_c.get(k).unwrap().clone())
            .collect::<Vec<usize>>()
            .as_slice()
            .try_into()
            .unwrap();

        add_boundary(&mut grid, xp, yp, x, y);
        add_boundary(&mut grid, xn, yn, x, y);
    }

    for i in 0..grid_size {
        flood(&mut grid, grid_size, i, 0);
        flood(&mut grid, grid_size, i, grid_size - 1);
        flood(&mut grid, grid_size, 0, i);
        flood(&mut grid, grid_size, grid_size - 1, i);
    }

    for i in 0..grid_size {
        for j in 0..grid_size {
            if grid[i][j] == 0 {
                grid[i][j] = 1;
            } else if grid[i][j] == 2 {
                grid[i][j] = 0
            }
        }
    }

    for i in 1..grid_size {
        grid[i][0] += grid[i - 1][0];
        grid[0][i] += grid[0][i - 1];
    }

    for i in 1..grid_size {
        for j in 1..grid_size {
            grid[i][j] += grid[i - 1][j] + grid[i][j - 1] - grid[i - 1][j - 1];
        }
    }

    let are_all_ones = |x1: usize, y1: usize, x2: usize, y2: usize| {
        let required = ((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)) as i64;
        let mut current = grid[x2][y2];

        if x1 > 0 {
            current -= grid[x1 - 1][y2];
        }

        if y1 > 0 {
            current -= grid[x2][y1 - 1];
        }

        if x1 > 0 && y1 > 0 {
            current += grid[x1 - 1][y1 - 1];
        }

        required == current
    };

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            let [x1, y1]: [usize; 2] = parsed[i]
                .iter()
                .map(|k| compressed_c.get(k).unwrap().clone())
                .collect::<Vec<usize>>()
                .as_slice()
                .try_into()
                .unwrap();

            let [x2, y2]: [usize; 2] = parsed[j]
                .iter()
                .map(|k| compressed_c.get(k).unwrap().clone())
                .collect::<Vec<usize>>()
                .as_slice()
                .try_into()
                .unwrap();

            if are_all_ones(min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)) {
                res = max(
                    res,
                    ((parsed[i][0] - parsed[j][0]).abs() + 1)
                        * ((parsed[i][1] - parsed[j][1]).abs() + 1),
                );
            }
        }
    }

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day09/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day09/input");
    part1(&lines);
    part2(&lines);
}
