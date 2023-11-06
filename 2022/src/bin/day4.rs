use std::ops::RangeInclusive;

#[path="../common.rs"]
mod common;

fn range_contained(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) ||
    b.contains(a.start()) || b.contains(a.end())
}

fn assignments_contained(assignments: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> usize {
    assignments.iter().map(|assignment| 
        { if range_contained(&assignment.0, &assignment.1) { return 1 } else { return 0}}).sum()
}

fn main() {
    let mut assignments = Vec::new();
    while let Some(strassignment) = common::read_line() {
        let splitted = strassignment.split(",").map(|str_range| {
            str_range.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();
        assignments.push((splitted[0][0]..=splitted[0][1], splitted[1][0]..=splitted[1][1]));
    }
    println!("Result {}", assignments_contained(&assignments));
}