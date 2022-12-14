use std::collections::HashMap;

#[path="../common.rs"]
mod common;

#[derive(Debug)]
enum Tile {
    Sand,
    Rock
}

fn str_to_point(strp: &str) -> common::Position {
    let spl = strp.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    common::Position::new(spl[0], spl[1])
}

fn expand_line(p1: &common::Position, p2: &common::Position) -> Vec<common::Position> {
    let mut result = Vec::new();
    if p1.x == p2.x {
        for y in p1.y..=p2.y {
            result.push(common::Position::new(p1.x, y));
        }
    } else {
        for x in p1.x..=p2.x {
            result.push(common::Position::new(x, p1.y));
        }
    }
    result
}


fn main() {
    let mut wall = HashMap::new();
    while let Some(line) = common::read_line() {
        let mut splitted = line.split(" -> ");
        let mut prev = splitted.next().unwrap();
        while let Some(cur) = splitted.next() {
            let l = expand_line(&str_to_point(prev), &str_to_point(cur));
            l.iter().for_each(|x| { wall.insert(x.to_owned(), Tile::Rock); });
            prev = cur;
        }
    }
    let maxy = wall.iter().map(|(k, _)| k.y).max().unwrap();
    println!("Read {:?}", wall);
    println!("Max y {}", maxy);
}