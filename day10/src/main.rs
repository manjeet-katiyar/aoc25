use good_lp::*;
use std::{cmp::min, i32::MAX};

fn solve1(indicator: i64, buttons: &Vec<i64>, idx: usize, current: i64) -> i64 {
    if idx >= buttons.len() {
        if current == indicator {
            return 0;
        }

        return MAX as i64;
    }

    let res1 = solve1(indicator, buttons, idx + 1, current);
    let res2 = 1 + solve1(indicator, buttons, idx + 1, current ^ buttons[idx]);

    return min(res1, res2);
}

fn part1(lines: &Vec<String>) {
    let parsed: Vec<(i64, Vec<i64>, Vec<usize>)> = lines
        .iter()
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            match tokens.as_slice() {
                [indicator, buttons @ .., joltage] => {
                    let set_bits: Vec<usize> = indicator
                        .trim_matches(['[', ']'])
                        .chars()
                        .enumerate()
                        .filter_map(|(idx, v)| if v == '#' { Some(idx) } else { None })
                        .collect();

                    let mut indicator: i64 = 0;
                    set_bits.iter().for_each(|v| indicator |= 1 << v);

                    let buttons: Vec<i64> = buttons
                        .iter()
                        .map(|s| {
                            let mut mask: i64 = 0;
                            s.trim_matches(['(', ')'])
                                .split(',')
                                .map(|n| n.parse::<usize>().unwrap())
                                .for_each(|v| mask |= 1 << v);
                            mask
                        })
                        .collect();

                    let joltage: Vec<usize> = joltage
                        .trim_matches(['{', '}'])
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect();

                    (indicator, buttons, joltage)
                }
                _ => panic!("Parse error"),
            }
        })
        .collect();

    let res: i64 = parsed
        .iter()
        .map(|(indicator, buttons, _joltage)| solve1(*indicator, buttons, 0, 0))
        .sum();

    println!("Part1: {}", res);
}

fn solve2(target: &[i32], updates: &[Vec<usize>]) -> Option<i32> {
    let mut vars = ProblemVariables::new();
    let m = updates.len();
    let n = target.len();

    let counts: Vec<_> = (0..m)
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    let objective: Expression = counts.iter().sum();

    let mut solver_builder = vars.minimise(objective).using(coin_cbc);

    for j in 0..n {
        let mut expr = Expression::from(0);
        for i in 0..m {
            if updates[i].contains(&j) {
                expr = expr + counts[i];
            }
        }
        solver_builder = solver_builder.with(constraint!(expr == target[j]));
    }

    let solution = solver_builder.solve();
    match solution {
        Ok(sol) => {
            let values: Vec<i32> = counts
                .iter()
                .map(|&v| sol.value(v).round() as i32)
                .collect();
            let total = values.iter().sum();
            Some(total)
        }
        Err(_) => None,
    }
}

fn part2(lines: &Vec<String>) {
    let parsed: Vec<(i64, Vec<Vec<usize>>, Vec<i32>)> = lines
        .iter()
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            match tokens.as_slice() {
                [indicator, buttons @ .., joltage] => {
                    let set_bits: Vec<usize> = indicator
                        .trim_matches(['[', ']'])
                        .chars()
                        .enumerate()
                        .filter_map(|(idx, v)| if v == '#' { Some(idx) } else { None })
                        .collect();

                    let mut indicator: i64 = 0;
                    set_bits.iter().for_each(|v| indicator |= 1 << v);

                    let buttons: Vec<Vec<usize>> = buttons
                        .iter()
                        .map(|s| {
                            s.trim_matches(['(', ')'])
                                .split(',')
                                .map(|n| n.parse::<usize>().unwrap())
                                .collect()
                        })
                        .collect();

                    let joltage: Vec<i32> = joltage
                        .trim_matches(['{', '}'])
                        .split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect();

                    (indicator, buttons, joltage)
                }
                _ => panic!("Parse error"),
            }
        })
        .collect();

    let res: i32 = parsed
        .iter()
        .map(|(_indicator, buttons, joltage)| solve2(&joltage, buttons).unwrap_or_default())
        .sum();

    println!("Part2: {}", res);
}

fn main() {
    let lines = utils::read_input("day10/input.test");
    part1(&lines);
    part2(&lines);

    let lines = utils::read_input("day10/input");
    part1(&lines);
    part2(&lines);
}
