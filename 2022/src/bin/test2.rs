#[path="../common.rs"]
mod common;
use common::*;
use std::collections::*;

#[derive(Debug)]
struct Tile {
    id: i32,
    data: Vec<String>
}

impl Tile {
    fn new(id: i32, data: &Vec<&str>) -> Tile {
        Tile {id: id, data: data.iter().map(|s| s.to_string()).collect() }
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
            self.data[i] = self.data[i].chars().rev().collect();
        }
    }
}


fn main() {
    if let Some(t) = Tile::read().as_mut() {
        println!("Read: {:?}", t);
        t.flip();
        println!("Flipped: {:?}", t);
    } 
    // while let Some(t) = Tile::read() {
    //     println!("Read tile {:?}", t);
    // }
    // println!("No more tiles");
}