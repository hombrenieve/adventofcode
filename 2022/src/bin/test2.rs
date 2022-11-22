#[path="../common.rs"]
mod common;
use common::*;
use std::{collections::*, borrow::BorrowMut};

#[derive(Debug)]
struct Tile {
    id: i32,
    data: Vec<Vec<char>>
}

impl Tile {
    fn new(id: i32, data: &Vec<&str>) -> Tile {
        Tile {id: id, data: data.iter().map(|s| s.chars().collect()).collect() }
    }

    fn read() -> Option<Tile> {
        if let Some(title) = read_line() {
            let id = title[..title.len()-1].split(' ').last().unwrap().parse::<i32>().unwrap();
            return Some(Tile::new(id, &read_until_empty_line().split(' ').collect()));
        }
        None
    }

    fn flip(&mut self) {
        for i in 0..self.data.len()-1 {
            self.data[i] = self.data[i].iter().rev().map(|c| c.to_owned()).collect();
        }
    }

    fn rotate(&mut self) {
        let mut new_data = self.data.to_owned();
        for i in 0..=self.data.len()-1 {
            for j in 0..=self.data[i].len()-1 {
                new_data[j][self.data[i].len()-1-i] = self.data[i][j];
            }
        }
        self.data = new_data;
    }
}


fn main() {
    let mut tiles = HashMap::new();
    while let Some(t) = Tile::read() {
        tiles.insert(t.id, t);
    }
    let t = tiles.get_mut(&2311).unwrap();
    println!("Acquired: {:?}", t);
    t.rotate();
    println!("After rotated inline: {:?}", tiles.get(&2311).unwrap());
}