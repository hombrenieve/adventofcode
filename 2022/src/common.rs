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

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position {
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

    pub fn from_str(strp: &str) -> Position {
        let spl = strp.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        Position::new(spl[0], spl[1])
    }

    pub fn distance(&self, other: &Self) -> i32 {
        cmp::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    pub fn line_to(&self, other: &Self) -> Vec<Position> {
        let r = |a,b| { if a < b { a..=b } else { b..=a }  };
        if self.x == other.x { //Vertical
            r(self.y, other.y).map(|y| Position::new(self.x, y)).collect()
        } else { //Horizontal
            r(self.x, other.x).map(|x| Position::new(x, self.y)).collect()
        }
    }
}
