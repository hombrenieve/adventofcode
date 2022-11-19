#[path="../common.rs"]
mod common;

fn calc_freq(changes: Vec<i32>) -> i32 {
    changes.iter().sum()
}

fn main() {
    let values = common::read_until_eof();
    let changes = common::to_ints(values.as_str(), " ");
    println!("Freqs are {}", calc_freq(changes));
}