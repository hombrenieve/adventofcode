use std::collections::HashMap;

#[path="../common.rs"]
mod common;

#[derive(Debug)]
enum Tile {
    Sand,
    Rock
}

const ONE_DOWN: common::Position = common::Position{x: 0, y: 1};
const ONE_DOWN_LEFT: common::Position = common::Position{x: -1, y: 1};
const ONE_DOWN_RIGHT: common::Position = common::Position{x: 1, y: 1};
const SAND_SOURCE: common::Position = common::Position{x: 500, y: 0};

fn str_to_point(strp: &str) -> common::Position {
    let spl = strp.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    common::Position::new(spl[0], spl[1])
}

fn expand_line(p1: &common::Position, p2: &common::Position) -> Vec<common::Position> {
    let mut result = Vec::new();
    if p1.x == p2.x {
        if p2.y < p1.y {
            for y in p2.y..=p1.y {
                result.push(common::Position::new(p1.x, y));
            }
        } else {
            for y in p1.y..=p2.y {
                result.push(common::Position::new(p1.x, y));
            }
        }
    } else if p2.x < p1.x {
        for x in p2.x..=p1.x {
            result.push(common::Position::new(x, p1.y));
        }
    } else {
        for x in p1.x..=p2.x {
            result.push(common::Position::new(x, p1.y));
        }
    }
    result
}

fn next_move(wall: &HashMap<common::Position, Tile>, cur: &common::Position) -> common::Position {
    if !wall.contains_key(&(cur.to_owned() + ONE_DOWN)) {
        return cur.to_owned() + ONE_DOWN;
    }
    if !wall.contains_key(&(cur.to_owned() + ONE_DOWN_LEFT)) {
        return cur.to_owned() + ONE_DOWN_LEFT;
    }
    if !wall.contains_key(&(cur.to_owned() + ONE_DOWN_RIGHT)) {
        return cur.to_owned() + ONE_DOWN_RIGHT;
    }
    cur.to_owned()
}

fn fill_with_sand(wall: &mut HashMap<common::Position, Tile>, ymax: i32) {
    let mut cur = SAND_SOURCE;
    while cur.y < ymax {
        let prev = cur.clone();
        cur = next_move(wall, &cur);
        if cur == prev {
            wall.insert(cur, Tile::Sand);
            cur = SAND_SOURCE;
        }
    }
}


fn main() {
    let mut wall = HashMap::new();
    while let Some(line) = common::read_line() {
        let mut splitted = line.split(" -> ");
        let mut prev = splitted.next().unwrap();
        while let Some(cur) = splitted.next() {
            let l = expand_line(&str_to_point(prev), &str_to_point(cur));
            l.iter().for_each(|x| { wall.insert(x.to_owned(), Tile::Rock); });
            prev = cur.clone();
        }
    }
    let maxy = wall.iter().map(|(k, _)| k.y).max().unwrap();
    println!("Max y {}", maxy);
    fill_with_sand(&mut wall, maxy);
    println!("Sand grains: {}", wall.iter().filter(|(_, v)| matches!(v, Tile::Sand)).collect::<Vec<_>>().len());
}