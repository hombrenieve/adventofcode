#[path="../common.rs"]
mod common;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Element {
    List(Vec<Element>),
    Num(i32)
}

fn adv_next_non_digit(elem: &str, current: &usize) -> (String, usize) {
    let mut chars = String::new();
    for i in current.to_owned()..elem.len() {
        if [',','[', ']'].contains(&elem.chars().nth(i).unwrap()) {
            return (chars, i);
        } else {
            chars.push(elem.chars().nth(i).unwrap());
        }
    }
    (chars, elem.len())
}

fn load_list(str_list: &str, current: &usize) -> (Element, usize) {
    let mut list = Vec::new();
    let mut i = current.to_owned();
    while i != str_list.len() {
        match str_list.chars().nth(i).unwrap() {
            ']' => {
                return (Element::List(list), i+1);
            },
            '[' => {
                let (elem, new_i) = load_list(str_list, &(i+1));
                list.push(elem);
                i = new_i;
            },
            ',' => i += 1,
            _ => {
                let (read, new_i) = adv_next_non_digit(str_list, &i);
                if let Ok(value) = read.parse::<i32>() {
                    list.push(Element::Num(value));
                }
                i = new_i;
            }
        }
    }
    panic!("List mismatched");
}

fn compare(left: &Element, right: &Element) -> Option<bool> {
    match (left, right) {
        (Element::Num(lvalue), Element::Num(rvalue)) => {
            if lvalue < rvalue { return Some(true); }
            if lvalue > rvalue { return Some(false); }
            return None;
        },
        (Element::List(llist), Element::List(rlist)) => {
            let mut llist_iter = llist.iter();
            let mut rlist_iter = rlist.iter();
            loop {
                if let Some(l) = llist_iter.next() {
                    if let Some(r) = rlist_iter.next() {
                        if let Some(result) = compare(l, r) {
                            return Some(result);
                        } else {
                            continue;// Continue
                        }
                    } else {
                        return Some(false); //The right list run out of items
                    }
                } else if rlist.len() > llist.len() {
                    return Some(true); //Left run out of elements
                }
                return None;                
            }
        },
        (Element::Num(lvalue), Element::List(rlist)) => {
            return compare(&Element::List(vec![Element::Num(lvalue.to_owned())]), &Element::List(rlist.to_owned()));
        },
        (Element::List(llist), Element::Num(rvalue)) => {
            return compare(&Element::List(llist.to_owned()), &Element::List(vec![Element::Num(rvalue.to_owned())]));
        }
    }
}

fn sort_cmp(left: &Element, right: &Element) -> std::cmp::Ordering {
    match compare(left, right) {
        None => std::cmp::Ordering::Equal,
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater
    }
}

fn main() {
    let mut packets = Vec::new();
    while let Some(line1) = common::read_line() {
        let line2 = common::read_line().unwrap();
        common::read_line();
        packets.push(load_list(&line1[..], &1).0);
        packets.push(load_list(&line2[..], &1).0);
    }
    let divider1 = Element::List(vec![Element::List(vec![Element::Num(2)])]);
    let divider2 = Element::List(vec![Element::List(vec![Element::Num(6)])]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort_by(|l, r| sort_cmp(l, r));
    for i in 0..packets.len() {
        println!("{} -> {:?}", i, packets[i]);
    }
    let in1 = packets.iter().position(|x| x.to_owned() == divider1).unwrap();
    let in2 = packets.iter().position(|x| x.to_owned() == divider2).unwrap();
    println!("Result is {}", (in1+1)*(in2+1));
}