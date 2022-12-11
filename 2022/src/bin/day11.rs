use std::collections::HashMap;

#[path="../common.rs"]
mod common;


struct Monkey<F, G>
{
    items: Vec<i32>,
    inspected: usize,
    operation: F,
    test: G
}

impl<F,G> std::fmt::Debug for Monkey<F,G> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Monkey [ items {:?}, inspected: {} ]", self.items, self.inspected)
    }
}


fn get_func_from_op(op: Vec<&str>) -> Box<dyn Fn(i32) -> i32> {
    if op[6] == "+" {
        if op[7] == "old" {
            return Box::new(|x| x + x);
        } else {
            let par = op[7].parse::<i32>().expect("Error parsing");
            return Box::new(move |x| x + par);
        }
    } else {
        if op[7] == "old" {
            return Box::new(|x| x * x);
        } else {
            let par = op[7].parse::<i32>().expect("Error parsing");
            return Box::new(move |x| x * par);
        }
    }
}

fn get_func_from_test(test: Vec<&str>) -> impl Fn(i32) -> usize {
    let div = test[5].parse::<i32>().expect("Error parsing");
    let trueb = test[15].parse::<usize>().expect("Error parsing");
    let falseb = test[25].parse::<usize>().expect("Error parsing");
    move |x| {
        match x % div {
            0 => trueb,
            _ => falseb
        }
    }
}


fn load() -> Monkey<impl Fn(i32) -> i32, impl Fn(i32) -> usize> {
    let starting = common::read_line().unwrap()[18..].split(", ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let op_str = common::read_line().unwrap();
    let op = op_str.split(' ').collect::<Vec<&str>>();
    let test_str = common::read_n_lines(3, " ");
    let test = test_str.split(' ').collect::<Vec<&str>>();
    Monkey {
        items: starting,
        inspected: 0,
        operation: get_func_from_op(op),
        test: get_func_from_test(test)
    }
}

fn play_turn(monkeys: &mut HashMap<usize, Monkey<impl Fn(i32) -> i32, impl Fn(i32) -> usize>>) {
    for i in 0..monkeys.len() {
        let mut monkey = monkeys.remove(&i).unwrap();
        while !monkey.items.is_empty() {
            let mut item = monkey.items.remove(0);
            item = (monkey.operation)(item);
            monkey.inspected += 1;
            item /= 3;
            let dest = (monkey.test)(item);
            if dest == i {
                monkey.items.push(item);
            } else {
                monkeys.get_mut(&dest).unwrap().items.push(item);
            }
        }
        monkeys.insert(i, monkey);
    }
}

fn monkey_bussiness<F,G>(monkeys: &HashMap<usize, Monkey<F,G>>) -> usize {
    let mut inspections = monkeys.iter().map(|(_,m)| m.inspected).collect::<Vec<usize>>();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}


fn main() {
    let mut monkeys = HashMap::new();
    let mut index: usize = 0;
    loop {
        if let Some(_) = common::read_line() {
            monkeys.insert(index, load());
            index += 1;
            common::read_line(); //dispose empty line
        } else { break; }
    }
    println!("Monkeys: {:?}", monkeys);
    for _ in 1..=20 { play_turn(&mut monkeys) }
    println!("Monkey bussiness value: {}", monkey_bussiness(&monkeys));
}