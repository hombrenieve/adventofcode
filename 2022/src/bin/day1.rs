#[path="../common.rs"]
mod common;


fn find_max(rations: &Vec<Vec<i32>>) -> i32 {
    rations.iter().map(|v| v.iter().sum()).max().unwrap()
}

fn find_top_three_sum(rations: &Vec<Vec<i32>>) -> i32 {
    let mut sums = rations.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
    sums.sort();
    sums.iter().rev().take(3).sum()
}


fn main() {
    let mut rations = Vec::new();
    loop {
        let strrat = common::read_until_empty_line();
        if strrat.is_empty() { break; }
        rations.push(common::to_ints(&strrat[..], " "));
    }
    println!("Top three sum: {}", find_top_three_sum(&rations));
}