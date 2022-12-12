use std::{
    cmp,
    ops::{Add, AddAssign},
};

pub fn read_line() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(n) => {
            if n == 0 {
                None
            } else {
                line.pop();
                Some(line)
            }
        }
        Err(_) => None,
    }
}

fn read_until_n(n: usize, sep: char) -> String {
    let mut line = String::new();
    let mut file = String::new();
    while let Ok(read) = std::io::stdin().read_line(&mut line) {
        if read == n {
            file.pop();
            return file;
        } else if read == 0 {
            return file;
        }
        line.pop();
        file.push_str(line.as_str());
        file.push(sep);
        line.clear();
    }
    //In case of Error
    file
}

pub fn read_until_empty_line() -> String {
    read_until_n(1, ' ')
}

pub fn read_until_eof() -> String {
    read_until_n(0, ' ')
}

pub fn read_n_lines(n: u32, sep: &str) -> String {
    let mut buffer = read_line().unwrap();
    for i in 1..n {
        buffer.push_str(sep);
        buffer.push_str(&read_line().unwrap()[..]);
    }
    buffer
}

pub fn to_ints(line: &str, sep: &str) -> Vec<i32> {
    line.split(sep).map(|x| x.parse::<i32>().unwrap()).collect()
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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
    pub fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }

    pub fn distance(&self, other: &Self) -> i32 {
        cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}
