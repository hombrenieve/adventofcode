use std::{collections::{HashSet, btree_set::Intersection, HashMap}, cmp::min_by};

#[path="../common.rs"]
mod common;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    open: bool,
    rate: i32,
    tunnels: Vec<String>
}

impl Valve{
    fn new(name: &str, rate: i32, tunnels: Vec<String>) -> Valve {
        Valve {
            name: String::from(name),
            open: false,
            rate: rate,
            tunnels: tunnels
        }
    }
}

enum Action {
    Open,
    Move(String),
    Stall
}

#[derive(Debug, Clone)]
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

    fn end_turn(&mut self) {
        self.minutes -= 1;
        self.pressure += self.releasing() as usize;
    }

    fn play_turn(&mut self, action: &Action) {
        match action {
            Action::Open => self.valves.get_mut(&self.current).unwrap().open = true,
            Action::Move(valve) => self.current = valve.to_owned(),
            Action::Stall => {}
        }
        self.end_turn();
    }
}

struct DTNode {
    action: Action,
    state: Volcano,
    weight: i32,
    visited: bool,
    children: Vec<DTNode>
}

impl DTNode {
    fn new(action: Action, initial_state: &Volcano) -> DTNode {
        let mut state = initial_state.to_owned();
        state.play_turn(&action);
        let weight = state.releasing();
        DTNode {
            action: action,
            state: state,
            weight: weight,
            visited: false,
            children: Vec::new()
        }
    }

    fn expand_dt(&mut self) {
        if !self.state.is_end() && self.children.is_empty() {
            let valve = self.state.valves.get(&self.state.current).unwrap();
            if !valve.open {
                self.children.push(DTNode::new(Action::Open, &self.state));
            }
            valve.tunnels.iter().for_each(|t| self.children.push(DTNode::new(Action::Move(t.to_owned()), &self.state)));
            self.children.push(DTNode::new(Action::Stall, &self.state));
        }
    }
}

fn get_best_play(volcano: &Volcano) -> usize {
    let mut best_play:usize = 0;
    let mut decision_tree = DTNode{ action: Action::Stall, state: volcano.to_owned(), weight: 0, visited: true, children: Vec::new()};
    let mut q = Vec::new();
    q.push(&mut decision_tree);
    while !q.is_empty() {
        let mut dt = q.pop().unwrap();
        dt.expand_dt();
        if dt.children.is_empty() {
            best_play = std::cmp::max(best_play, dt.state.pressure);
        }
        for ndt in &mut dt.children {
            if !ndt.visited {
                ndt.visited = true;
                q.push(ndt);
            }
        }
    }
    best_play
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
    println!("Read: {:?}", volcano);
    println!("Best play: {}", get_best_play(&volcano));
}