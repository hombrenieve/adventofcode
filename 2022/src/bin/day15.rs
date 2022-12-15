use std::collections::HashSet;

#[path="../common.rs"]
mod common;


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

impl Sensor {
    fn exclusion_zone(&self, row: i64) -> Vec<Point> {
        let mut excl = Vec::new();
        let md = self.position.manhattan(&self.beacon);
        let mid = Point{ x: self.position.x, y: row};
        if self.position.manhattan(&mid) <= md {
            excl.push(mid);
            let mut delta = 1;
            loop {
                let pr = Point { x: self.position.x+delta, y: row};
                let pl = Point { x: self.position.x-delta, y: row};
                if self.position.manhattan(&pr) <= md {
                    excl.push(pr); excl.push(pl);
                } else {
                    break;
                }
                delta += 1;
            }
        }
        //Remove beacon
        if let Some(id) = excl.iter().position(|p| *p == self.beacon) {
            excl.remove(id);
        }
        excl
    }
}

fn no_beacon_possible(sensors: &Vec<Sensor>, row: i64) -> usize {
    let mut exclusions = HashSet::new();
    for s in sensors {
        s.exclusion_zone(row).iter().for_each(|p| { exclusions.insert(p.to_owned()); });
    }
    exclusions.len()
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
    println!("Exclusions: {}", no_beacon_possible(&sensors, 2000000));
}