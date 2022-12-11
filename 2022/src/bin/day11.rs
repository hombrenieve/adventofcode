use std::collections::HashMap;

#[path="../common.rs"]
mod common;

const LCM: usize = 9699690; //This is manually calculated from my input (could be automated but..)

struct Monkey<F, G>
{
    items: Vec<usize>,
    inspected: usize,
    operation: F,
    test: G
}

impl<F,G> std::fmt::Debug for Monkey<F,G> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Monkey [ items {:?}, inspected: {} ]", self.items, self.inspected)
    }
}


fn get_func_from_op(op: Vec<&str>) -> Box<dyn Fn(usize) -> usize> {
    if op[6] == "+" {
        if op[7] == "old" {
            return Box::new(|x| (x + x) % LCM);
        } else {
            let par = op[7].parse::<usize>().expect("Error parsing");
            return Box::new(move |x| (x + par) % LCM);
        }
    } else {
        if op[7] == "old" {
            return Box::new(|x| ((x as u128 * x as u128) % LCM as u128) as usize);
        } else {
            let par = op[7].parse::<usize>().expect("Error parsing");
            return Box::new(move |x| (x * par) % LCM);
        }
    }
}

fn get_func_from_test(test: Vec<&str>) -> impl Fn(usize) -> usize {
    let div = test[5].parse::<usize>().expect("Error parsing");
    let trueb = test[15].parse::<usize>().expect("Error parsing");
    let falseb = test[25].parse::<usize>().expect("Error parsing");
    move |x| {
        match x % div {
            0 => trueb,
            _ => falseb
        }
    }
}


fn load() -> Monkey<impl Fn(usize) -> usize, impl Fn(usize) -> usize> {
    let starting = common::read_line().unwrap()[18..].split(", ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
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

fn play_turn(monkeys: &mut HashMap<usize, Monkey<impl Fn(usize) -> usize, impl Fn(usize) -> usize>>) {
    for i in 0..monkeys.len() {
        let mut monkey = monkeys.remove(&i).unwrap();
        while !monkey.items.is_empty() {
            let mut item = monkey.items.remove(0);
            item = (monkey.operation)(item);
            monkey.inspected += 1;
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
    for _ in 1..=10000 { play_turn(&mut monkeys) }
    println!("Monkey bussiness value: {}", monkey_bussiness(&monkeys));
}