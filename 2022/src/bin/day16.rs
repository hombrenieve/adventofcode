use std::{collections::{HashSet, btree_set::Intersection, HashMap}, cmp::min_by};

#[path="../common.rs"]
mod common;

#[derive(Debug)]
struct Metadata {
    visited: bool,
    cost: i32,
    gain: i32
}

impl Metadata {
    fn clear(&mut self) {
        self.visited = false;
        self.cost = 0;
        self.gain = 0;
    }

    fn update(&mut self, minutes: i32, cost: i32, rate: i32) {
        self.cost = cost;
        self.gain = (minutes - cost) * rate;
        self.visited = true;
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    open: bool,
    rate: i32,
    tunnels: Vec<String>,
    meta: Metadata
}

impl Valve{
    fn new(name: &str, rate: i32, tunnels: Vec<String>) -> Valve {
        Valve {
            name: String::from(name),
            open: false,
            rate: rate,
            tunnels: tunnels,
            meta: Metadata { visited: false, cost: 0, gain: 0 }
        }
    }
}

#[derive(Debug)]
struct Volcano {
    minutes: i32,
    pressure: usize,
    current: String,
    valves: HashMap<String, Valve>
}

impl Volcano {
    fn releasing(&self) -> i32 {
        self.valves.iter().map(|(_, v)| if v.open { v.rate } else { 0 }).sum()
    }

    fn is_end(&self) -> bool {
        self.minutes == 0
    }

    fn execute(&mut self) {
        //Get best action
        let mut v_gain = self.valves.iter().filter(|(_, v)| v.meta.cost <= self.minutes).map(|(k, v)| (k.to_owned(), v.meta.gain.to_owned())).collect::<Vec<(String, i32)>>();
        v_gain.sort_by_key(|(_, g)| *g);
        let v = self.valves.get_mut(&v_gain.last().unwrap().0).unwrap();
        //println!("Time {}, Go {}, Cost {}, Gain {}", self.minutes, v.name, v.meta.cost, v.meta.gain);
        println!("Gains: {:?}", v_gain);
        self.minutes -= v.meta.cost;
        self.current = v.name.to_owned();
        v.open = true;
        self.pressure += v.meta.gain as usize;
    }

    fn calculate_gains(&mut self) {
        //First clear metadata
        self.valves.iter_mut().for_each(|(_, v)| v.meta.clear());
        //BFS calculating gain
        let mut q = Vec::new();
        let mut v = self.valves.remove(&self.current).unwrap();
        if !v.open {
            v.meta.update(self.minutes, 1, v.rate);
        } else {
            v.meta.update(self.minutes, 1, 0);
        }
        q.push(v);
        while !q.is_empty() {
            let v = q.pop().unwrap();
            for ch in &v.tunnels {
                if let Some(mut vt) = self.valves.remove(ch) {
                    if !vt.meta.visited {
                        if !vt.open {
                            vt.meta.update(self.minutes, v.meta.cost+1, vt.rate);
                        } else {
                            vt.meta.update(self.minutes, v.meta.cost, 0);
                        }
                        q.push(vt);
                    } else {
                        self.valves.insert(vt.name.to_owned(), vt);
                    }
                }
            }
            self.valves.insert(v.name.to_owned(), v);
        }
    }

    fn play(&mut self) {
        while !self.is_end() {
            self.calculate_gains();
            //println!("After gains: {:?}", self.valves);
            self.execute();
        }
    }
}

fn main() {
    let mut volcano = Volcano { minutes: 30, pressure: 0, current: String::from(""), valves: HashMap::new()};
    while let Some(line) = common::read_line() {
        let mut tokens = line.replace("Valve ", "");
        tokens = tokens.replace("rate=", "");
        tokens = tokens.replace("; ", " ");
        tokens = tokens.replace(", ", ",");
        let mut sp = tokens.split(' ');
        let name = sp.next().unwrap();
        sp.next(); sp.next();
        let rate = sp.next().unwrap().parse::<i32>().unwrap();
        sp.next(); sp.next();sp.next(); sp.next();
        let tunnels = sp.next().unwrap().split(',').map(|t| String::from(t)).collect::<Vec<String>>();
        volcano.valves.insert(String::from(name), Valve::new(name, rate, tunnels));
        if volcano.current.is_empty() {
            volcano.current = String::from(name);
        }
    }
    //println!("Read: {:?}", volcano);
    volcano.play();
    println!("Pressure released: {}", volcano.pressure);
}