#[path="../common.rs"]
mod common;

fn calc_freq(changes: &Vec<i32>) -> i32 {
    changes.iter().sum()
}

fn get_repeated_freq(changes: &Vec<i32>) -> i32 {
    let mut visited = std::collections::HashSet::new();
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
    let values = common::read_until_eof();
    let changes = common::to_ints(values.as_str(), " ");
    println!("First repeated is {}", get_repeated_freq(&changes));
}