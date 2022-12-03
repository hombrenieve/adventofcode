#[path="../common.rs"]
mod common;

fn find_match(comp1: &Vec<char>, comp2: &Vec<char>) -> char {
    for c in comp2 {
        if comp1.contains(c) {
            return c.to_owned();
        }
    }
    '0'
}

fn get_priority(c: &char) -> u32 {
    let ascii = *c as u32;
    match ascii {
        97..=122 => ascii-96,
        65..=90 => ascii-38,
        _ => 0
    }
}

fn main() {
    let mut rucksacs = Vec::new();
    while let Some(strrack) = common::read_line() {
        rucksacs.push((strrack[..strrack.len()/2].chars().collect::<Vec<char>>(), strrack[strrack.len()/2..].chars().collect::<Vec<char>>()));
    }
    println!("Read: {:?}", rucksacs);
    println!("Priorities: {:?}", rucksacs.iter().map(|comp| { find_match(&comp.0, &comp.1)}).map(|c| { get_priority(&c)}).sum::<u32>());
}