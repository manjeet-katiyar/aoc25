use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part1(lines: &Vec<String>) {
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut res = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let neighbor_count = DIRECTIONS
                    .iter()
                    .filter_map(|&(dx, dy)| {
                        let new_i = i.checked_add_signed(dx)?;
                        let new_j = j.checked_add_signed(dy)?;

                        grid.get(new_i)?.get(new_j)
                    })
                    .filter(|&&c| c == '@')
                    .count();

                if neighbor_count < 4 {
                    res += 1;
                }
            }
        }
    }

    println!("Part1: {}", res);
}

fn part2(lines: &Vec<String>) {
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let n = grid.len();
    let m = grid[0].len();

    let mut res = 0;
    let mut visited = vec![vec![(false, 0); m]; n];
    let mut min_heap: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let neighbor_count = DIRECTIONS
                    .iter()
                    .filter_map(|&(dx, dy)| {
                        let new_i = i.checked_add_signed(dx)?;
                        let new_j = j.checked_add_signed(dy)?;

                        grid.get(new_i)?.get(new_j)
                    })
                    .filter(|&&c| c == '@')
                    .count();

                min_heap.push(Reverse((neighbor_count, i, j)));
                visited[i][j] = (false, neighbor_count);
            }
        }
    }

    while let Some(&Reverse((count, x, y))) = min_heap.peek() {
        min_heap.pop();
        if count >= 4 {
            break;
        }

        let (is_visited, _score) = visited[x][y];
        if is_visited {
            continue;
        }

        visited[x][y].0 = true;
        res += 1;
        DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| {
                let new_i = x.checked_add_signed(dx)?;
                let new_j = y.checked_add_signed(dy)?;
                grid.get(new_i)?.get(new_j)?;
                Some((new_i, new_j))
            })
            .for_each(|(ni, ny)| {
                if grid[ni][ny] == '@' {
                    let (is_visited_n, score_n) = visited[ni][ny];
                    if is_visited_n == false && score_n >= 4 {
                        visited[ni][ny].1 = score_n - 1;
                        min_heap.push(Reverse((score_n - 1, ni, ny)));
                    }
                }
            });
    }

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day04/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day04/input");
    part1(&lines);
    part2(&lines);
}
