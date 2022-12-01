#[path="../common.rs"]
mod common;


fn find_max(rations: &Vec<Vec<i32>>) -> i32 {
    rations.iter().map(|v| v.iter().sum()).max().unwrap()
}


fn main() {
    let mut rations = Vec::new();
    loop {
        let strrat = common::read_until_empty_line();
        if strrat.is_empty() { break; }
        rations.push(common::to_ints(&strrat[..], " "));
    }
    println!("Max: {}", find_max(&rations));
}