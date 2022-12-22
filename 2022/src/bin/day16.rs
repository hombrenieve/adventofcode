use std::{collections::{HashSet, btree_set::Intersection, HashMap}, cmp::min_by};

#[path="../common.rs"]
mod common;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Volcano<'a> {
    minutes: i32,
    pressure: usize,
    current: String,
    valves: &'a HashMap<String, Valve>, //No write
    meta: HashMap<String, Metadata>
}

impl<'a> Volcano<'a> {
    fn is_end(&self) -> bool {
        self.minutes == 0
    }

    fn execute(&mut self, tunnel: &String) -> usize {
        //Get best action
        let m = self.meta.get_mut(tunnel).unwrap();
        //println!("Time {}, Go {}, Cost {}, Gain {}", self.minutes, v.name, v.meta.cost, v.meta.gain);
        //println!("Gains: {}", m.gain);
        self.minutes -= m.cost;
        self.current = tunnel.to_owned();
        m.open = true;
        self.pressure += m.gain as usize;
        self.play()
    }

    fn calculate_gains(&mut self) {
        //First clear metadata
        self.meta.iter_mut().for_each(|(_, m)| m.clear());
        //BFS calculating gain
        let mut q = Vec::new();
        let v = self.valves.get(&self.current).unwrap();
        let mut m = self.meta.remove(&self.current).unwrap();
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
                    if let Some(mut mt) = self.meta.remove(&vt.name) {
                        if !mt.visited {
                            if !mt.open {
                                mt.update(self.minutes, m.cost+1, vt.rate);
                            } else {
                                mt.update(self.minutes, m.cost, 0);
                            }
                            q.push((vt, mt));
                        } else {
                            self.meta.insert(vt.name.to_owned(), mt);
                        }
                    }
                }
            }
            self.meta.insert(v.name.to_owned(), m);
        }
    }

    fn get_meaningful_actions(&self) -> Vec<String> {
        let mut v_gain = self.meta.iter().filter(|(_, m)| m.cost <= self.minutes).map(|(k, m)| (k.to_owned(), m.gain.to_owned())).collect::<Vec<(String, i32)>>();
        v_gain.sort_by_key(|(_, g)| *g);
        v_gain.iter().filter(|(_,g)| *g > 0).map(|(k, _)| k.to_owned()).collect()
    }

    fn play(&mut self) -> usize {
        if self.is_end() {
            return self.pressure;
        }
        self.calculate_gains();
        let mut results = Vec::new();
        for t in self.get_meaningful_actions() {
            results.push(self.clone().execute(&t));
        }
        if let Some(m) = results.iter().max() {
            return *m;
        }
        let c = self.current.to_owned();
        self.execute(&c)
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
    let mut volcano = Volcano { minutes: 30, pressure: 0, current: starting, valves: &valves,
    meta: valves.iter().map(|(k,_)| (k.to_owned(), Metadata::new())).collect::<HashMap<String, Metadata>>()};
    //println!("Read: {:?}", volcano);
    
    println!("Pressure released: {}", volcano.play());
}