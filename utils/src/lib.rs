use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod dsu;

pub fn read_input(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().filter_map(|line| line.ok()).collect()
}
