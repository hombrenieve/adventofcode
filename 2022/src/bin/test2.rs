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
    fn from(num: usize) -> Side {
        match num {
            0 => Side::Up,
            1 => Side::Down,
            2 => Side::Left,
            _ => Side::Right
        }
    }
}

#[derive(Debug)]
struct Tile {
    id: i32,
    data: Vec<Vec<char>>,
    neighbors: HashSet<i32>
}

impl Tile {
    fn new(id: i32, data: &Vec<&str>) -> Tile {
        Tile {id: id, 
            data: data.iter().map(|s| s.chars().collect()).collect(),
            neighbors: HashSet::new()
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
        for local_side in 0..3 {
            for other_side in 0..3 {
                if self.get_side(Side::from(local_side)) == other.get_side(Side::from(other_side)) {
                    self.neighbors.insert(other.id);
                    other.neighbors.insert(self.id);
                    return true;
                }
            }
        }
        false
    }

    fn num_neighbors(&self) -> usize {
        self.neighbors.len()
    }

    fn colocated(&self) -> bool {
        self.num_neighbors() > 0
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

fn do_match(t: &mut Tile, o: &mut Tile) -> bool {
    if t.match_with(o) {
        return true;
    }
    for _ in 1..5 {
        t.rotate();
        if t.match_with(o) {
            return true;
        }
    }
    t.flip();
    if t.match_with(o) {
        return true;
    }
    for _ in 1..5 {
        t.rotate();
        if t.match_with(o) {
            return true;
        }
    }

    t.flip();
    false
}

fn main() {
    let mut tiles = Vec::new();
    let mut matched = Vec::new();
    while let Some(t) = Tile::read() {
        tiles.push(t);
    }
    while tiles.len() > 0 {
        let mut t = tiles.pop().unwrap();
        for o in &mut tiles {
            do_match(t.borrow_mut(), o);
        }
        for o in &mut matched {
            do_match(t.borrow_mut(), o);
        }
        matched.push(t);
    }
    for t in matched {
        println!("Id: {}, neighbours: {:?}", t.id, t.neighbors);
    }
}