use std::{ops::{Add, AddAssign}, cmp, collections::HashSet};

#[path="../common.rs"]
mod common;

#[derive(Debug, Clone, Eq, Hash)]
struct Position {
    x: i32,
    y: i32
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y
        }
    }

    fn distance(&self, other: &Self) -> i32 {
        cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn new(m: &str) -> Direction {
        match m {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Can't match with direction")
        }
    }
    fn dir_vector(&self) -> Position {
        match self {
            Direction::Up => Position::new(0, 1),
            Direction::Down => Position::new(0, -1),
            Direction::Left => Position::new(-1, 0),
            Direction::Right => Position::new(1, 0)
        }
    }
    fn end_in_dir(&self, orig: &Position, steps: i32) -> Position {
        match self {
            Direction::Up => orig.clone() + Position::new(0, steps),
            Direction::Down => orig.clone() + Position::new(0, -steps),
            Direction::Left => orig.clone() + Position::new(-steps, 0),
            Direction::Right => orig.clone() + Position::new(steps, 0)
        }
    }
}

fn step_closer(a: i32, b: i32) -> i32 {
    if a > b {
        return b+1;
    } else {
        return b-1;
    }
}

fn calculate_tail_position(head_pos: &Position, tail_prev_pos: &Position) -> Position {
    if head_pos.distance(tail_prev_pos) <= 1 {
        return tail_prev_pos.clone();
    }
    match (head_pos.x == tail_prev_pos.x, head_pos.y == tail_prev_pos.y) {
        (true, false) => Position::new(tail_prev_pos.x , step_closer(head_pos.y, tail_prev_pos.y)),
        (false, true) => Position::new(step_closer(head_pos.x, tail_prev_pos.x) , tail_prev_pos.y),
        (false, false) => Position::new(step_closer(head_pos.x, tail_prev_pos.x), step_closer(head_pos.y, tail_prev_pos.y)),
        _ => tail_prev_pos.clone()
    }
}

fn calculate_tail_trail(head_trail: &Vec<Position>) -> HashSet<Position> {
    let mut ttrail = HashSet::new();
    let mut current = Position::new(0,0);
    for head_pos in head_trail {
        let new_pos = calculate_tail_position(head_pos, &current);
        ttrail.insert(new_pos.clone());
        current = new_pos;
    }
    ttrail
}

fn main() {
    let mut htrail = Vec::new();
    let mut orig = Position::new(0, 0);
    while let Some(line) = common::read_line() {
        let ls = line.split(" ").collect::<Vec<&str>>();
        let dir = Direction::new(ls[0]);
        let dir_vec = dir.dir_vector();
        let end_pos = dir.end_in_dir(&orig, ls[1].parse::<i32>().expect("Error parsing"));
        let mut trail = orig.clone();
        while trail != end_pos {
            htrail.push(trail.clone());
            trail += dir_vec.clone();
        }
        orig = trail;
    }
    htrail.push(orig);
    let ttrail = calculate_tail_trail(&htrail);
    println!("Tail pos: {}", ttrail.len());
}