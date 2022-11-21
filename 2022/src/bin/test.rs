#[path="../common.rs"]
mod common;
use common::*;
use std::collections::*;

fn calc_freq(changes: &Vec<i32>) -> i32 {
    changes.iter().sum()
}

fn get_repeated_freq(changes: &Vec<i32>) -> i32 {
    let mut visited = HashSet::new();
    let mut current = 0;
    loop {
        for change in changes {
            if !visited.insert(current) {
                return current;
            }
            current += change;
        }
    }
}

fn main() {
    let values = read_until_eof();
    let changes = to_ints(values.as_str(), " ");
    println!("First repeated is {}", get_repeated_freq(&changes));
}