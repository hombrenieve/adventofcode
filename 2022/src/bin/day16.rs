use std::{collections::{HashSet, btree_set::Intersection, HashMap}, cmp::min_by};

#[path="../common.rs"]
mod common;

#[derive(Debug)]
struct Metadata {
    open: bool,
    visited: bool,
    cost: i32,
    gain: i32
}

impl Metadata {
    fn new() -> Metadata {
        Metadata { open: false, visited: false, cost: 0, gain: 0 }
    }

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
    rate: i32,
    tunnels: Vec<String>
}

impl Valve{
    fn new(name: &str, rate: i32, tunnels: Vec<String>) -> Valve {
        Valve {
            name: String::from(name),
            rate: rate,
            tunnels: tunnels
        }
    }
}

#[derive(Debug)]
struct Volcano<'a> {
    minutes: i32,
    pressure: usize,
    current: String,
    valves: &'a HashMap<String, Valve>, //No write
}

impl<'a> Volcano<'a> {
    fn is_end(&self) -> bool {
        self.minutes == 0
    }

    fn execute(&mut self, meta: &mut HashMap<String, Metadata>) {
        //Get best action
        let mut v_gain = meta.iter().filter(|(_, m)| m.cost <= self.minutes).map(|(k, m)| (k.to_owned(), m.gain.to_owned())).collect::<Vec<(String, i32)>>();
        v_gain.sort_by_key(|(_, g)| *g);
        let m = meta.get_mut(&v_gain.last().unwrap().0).unwrap();
        //println!("Time {}, Go {}, Cost {}, Gain {}", self.minutes, v.name, v.meta.cost, v.meta.gain);
        println!("Gains: {:?}", v_gain);
        self.minutes -= m.cost;
        self.current = v_gain.last().unwrap().0.to_owned();
        m.open = true;
        self.pressure += m.gain as usize;
    }

    fn calculate_gains(&mut self, meta: &mut HashMap<String, Metadata>) {
        //First clear metadata
        meta.iter_mut().for_each(|(_, m)| m.clear());
        //BFS calculating gain
        let mut q = Vec::new();
        let v = self.valves.get(&self.current).unwrap();
        let mut m = meta.remove(&self.current).unwrap();
        if !m.open {
            m.update(self.minutes, 1, v.rate);
        } else {
            m.update(self.minutes, 1, 0);
        }
        q.push((v, m));
        while !q.is_empty() {
            let (v, m) = q.pop().unwrap();
            for ch in &v.tunnels {
                if let Some(vt) = self.valves.get(ch) {
                    if let Some(mut mt) = meta.remove(&vt.name) {
                        if !mt.visited {
                            if !mt.open {
                                mt.update(self.minutes, m.cost+1, vt.rate);
                            } else {
                                mt.update(self.minutes, m.cost, 0);
                            }
                            q.push((vt, mt));
                        } else {
                            meta.insert(vt.name.to_owned(), mt);
                        }
                    }
                }
            }
            meta.insert(v.name.to_owned(), m);
        }
    }

    fn play(&mut self) {
        let mut meta = self.valves.iter().map(|(k,_)| (k.to_owned(), Metadata::new())).collect::<HashMap<String, Metadata>>();
        while !self.is_end() {
            self.calculate_gains(&mut meta);
            //println!("After gains: {:?}", self.valves);
            self.execute(&mut meta);
        }
    }
}

fn main() {
    let mut valves = HashMap::new();
    let mut starting = String::from("");
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
        valves.insert(String::from(name), Valve::new(name, rate, tunnels));
        if starting.is_empty() {
            starting = String::from(name);
        }
    }
    let mut volcano = Volcano { minutes: 30, pressure: 0, current: starting, valves: &valves};
    //println!("Read: {:?}", volcano);
    volcano.play();
    println!("Pressure released: {}", volcano.pressure);
}