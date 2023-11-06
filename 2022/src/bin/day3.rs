#[path="../common.rs"]
mod common;

fn find_match(comp1: &Vec<char>, comp2: &Vec<char>, comp3: &Vec<char>) -> char {
    for c in comp3 {
        if comp1.contains(c) && comp2.contains(c) {
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
        rucksacs.push(strrack.chars().collect::<Vec<char>>());
    }
    let mut prios = Vec::new();
    for i in (0..rucksacs.len()).step_by(3) {
        prios.push(get_priority(&find_match(&rucksacs[i], &rucksacs[i+1], &rucksacs[i+2])));
    }
    println!("Priorities: {}", prios.iter().sum::<u32>());
}