#[path="../common.rs"]
mod common;

const MARKER_SIZE: usize = 14;

fn has_repeated_chars(msg: &str) -> bool {
    for i in 0..msg.len() {
        let c = msg.chars().nth(i).unwrap();
        for j in i+1..msg.len() {
            let c2 = msg.chars().nth(j).unwrap();
            if c == c2 {
                return true;
            }
        }
    }
    false
}

fn find_marker_after_character(msg: &str) -> usize {
    for pos in 0..msg.len() {
        let marker = &msg[pos..pos+MARKER_SIZE];
        if !has_repeated_chars(marker) {
            return pos+MARKER_SIZE;
        }
    }
    0
}

fn main() {
    let msg = common::read_line().unwrap();
    println!("Marker after char: {}", find_marker_after_character(&msg[..]));
}