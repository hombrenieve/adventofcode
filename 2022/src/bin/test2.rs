#[path="../common.rs"]
mod common;
use common::*;
use std::{collections::*, borrow::BorrowMut};

enum Side {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3
}

impl Side {
    fn i(self) -> usize {
        self as usize
    }
}

#[derive(Debug)]
struct Tile {
    id: i32,
    data: Vec<Vec<char>>,
    neighbors: [i32; 4]
}

impl Tile {
    fn new(id: i32, data: &Vec<&str>) -> Tile {
        Tile {id: id, 
            data: data.iter().map(|s| s.chars().collect()).collect(),
            neighbors: [-1; 4]
        }
    }

    fn read() -> Option<Tile> {
        if let Some(title) = read_line() {
            let id = title[..title.len()-1].split(' ').last().unwrap().parse::<i32>().unwrap();
            return Some(Tile::new(id, &read_until_empty_line().split(' ').collect()));
        }
        None
    }

    //Use closure to avoid copying vector
    fn get_side(&self, s: Side) -> Vec<char> {
        match s {
            Side::Up => self.data[0].to_owned(),
            Side::Down => self.data.last().unwrap().to_owned(),
            Side::Left => self.data.iter().map(|row| row[0]).collect::<Vec<char>>(),
            Side::Right => self.data.iter().map(|row| row.last().unwrap().to_owned()).collect::<Vec<char>>()
        }
    }

    fn match_with(&mut self, other: &mut Tile) -> bool {
        false
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
    //t.rotate();
    //println!("After rotated inline: {:?}", tiles.get(&2311).unwrap());
    println!("Up: {:?}, Down: {:?}, Left: {:?}, Right: {:?}", t.get_side(Side::Up), t.get_side(Side::Down), t.get_side(Side::Left), t.get_side(Side::Right));
}