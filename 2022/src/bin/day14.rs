use std::collections::HashMap;

#[path="../common.rs"]
mod common;
use common::Position;

#[derive(Debug)]
enum Tile {
    Sand,
    Rock
}

const ONE_DOWN: Position = Position{x: 0, y: 1};
const ONE_DOWN_LEFT: Position = Position{x: -1, y: 1};
const ONE_DOWN_RIGHT: Position = Position{x: 1, y: 1};
const SAND_SOURCE: Position = Position{x: 500, y: 0};

fn next_move(wall: &HashMap<Position, Tile>, floor: i32, cur: Position) -> Position {
    if (&cur + &ONE_DOWN).y == floor { return cur; }
    if !wall.contains_key(&(&cur + &ONE_DOWN)) {
        return &cur + &ONE_DOWN;
    }
    if !wall.contains_key(&(&cur + &ONE_DOWN_LEFT)) {
        return &cur + &ONE_DOWN_LEFT;
    }
    if !wall.contains_key(&(&cur + &ONE_DOWN_RIGHT)) {
        return &cur + &ONE_DOWN_RIGHT;
    }
    cur
}

fn fill_with_sand(wall: &mut HashMap<Position, Tile>, floor: i32) {
    let mut cur = SAND_SOURCE;
    loop {
        let prev = cur.clone();
        cur = next_move(wall, floor, cur);
        if cur == prev {
            wall.insert(cur, Tile::Sand);
            if prev == SAND_SOURCE { break; }
            cur = SAND_SOURCE;
        }
    }
}


fn main() {
    let mut wall = HashMap::new();
    while let Some(line) = common::read_line() {
        let mut splitted = line.split(" -> ");
        let mut prev = Position::from_str(splitted.next().unwrap());
        while let Some(strcur) = splitted.next() {
            let cur = Position::from_str(strcur);
            let l = prev.line_to(&cur);
            l.iter().for_each(|x| { wall.insert(x.to_owned(), Tile::Rock); });
            prev = cur;
        }
    }
    let maxy = wall.iter().map(|(k, _)| k.y).max().unwrap();
    println!("Max y {}", maxy);
    fill_with_sand(&mut wall, maxy+2);
    println!("Sand grains: {}", wall.iter().filter(|(_, v)| matches!(v, Tile::Sand)).collect::<Vec<_>>().len());
}