use std::{collections::{HashSet, btree_set::Intersection}, cmp::min_by};

#[path="../common.rs"]
mod common;

const MIN: i64 = 0;
const MAX: i64 = 4000000;
//const MAX: i64 = 20;

#[derive(Debug, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point{
    pub fn from_str(strp: &str) -> Point {
        let spl = strp.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        Point {
            x: spl[0],
            y: spl[1]
        }
    }

    fn manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point
}

type Interval = (i64, i64);

impl Sensor {
    fn exclusion_interval(&self, row: i64) -> Option<Interval> {
        let md = self.position.manhattan(&self.beacon);
        let mid = Point{ x: self.position.x, y: row};
        let midmd = self.position.manhattan(&mid);
        if self.position.manhattan(&mid) <= md {
            let delta = md - midmd;
            return Some((std::cmp::max(self.position.x-delta, MIN), std::cmp::min(self.position.x+delta, MAX)));
        }
        None
    }
}

fn superinterval(i1: &Interval, i2: &Interval) -> Option<Interval> {
    if i2.0 > i1.1 || i1.0 > i2.1 {
        return None;
    }
    Some((std::cmp::min(i1.0, i2.0), std::cmp::max(i1.1, i2.1)))
}

fn point_in_middle(i1: &Interval, i2: &Interval) -> i64 {
    if i1.0 < i2.0 {
        i1.1+1
    } else {
        i2.1+1
    }
}

fn reduce_intervals(intervals: &Vec<Interval>) -> Vec<Interval> {
    let mut result = intervals.clone();
    let mut result1 = Vec::new();
    loop {
        let mut interval = result[0];
        for i in 1..result.len() {
            if let Some(int) = superinterval(&interval, &result[i]) {
                interval = int;
            } else {
                result1.push(result[i]);
            }
        }
        result1.push(interval);
        if result.len() == result1.len() {
            return result1;
        } else {
            result = result1;
            result1 = Vec::new();
        }
    }
    
}

fn find_beacon(sensors: &Vec<Sensor>) -> Point {
    for row in MIN..=MAX {
        let mut ints = Vec::new();
        sensors.iter().for_each(|s| { 
            if let Some(int) = s.exclusion_interval(row) {
                ints.push(int);
            }
        });
        let reduced = reduce_intervals(&ints);
        if reduced.len() > 1 {
            return Point{ x: point_in_middle(&reduced[0], &reduced[1]), y: row };
        }
    }
    panic!("Can't find a result");
}


fn main() {
    let mut sensors = Vec::new();
    while let Some(line) = common::read_line() {
        let mut tokens = line.replace("Sensor at x=", "");
        tokens = tokens.replace(" closest beacon is at x=", "");
        tokens = tokens.replace(" y=", "");
        let mut ps = tokens.split(":").map(|x| Point::from_str(x)).collect::<Vec<Point>>();
        sensors.push(Sensor{position: ps.remove(0), beacon: ps.remove(0)});
    }
    let p = find_beacon(&sensors);
    println!("Point: {:?}", p);
    println!("Freq: {}", p.x*4000000+p.y);
}