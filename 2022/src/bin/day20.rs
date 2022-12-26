#[path="../common.rs"]
mod common;

#[derive(Debug)]
struct Element {
    num: i32,
    prev: usize,
    next: usize
}

impl Element {
    fn new(num: i32, prev: usize) -> Element {
        Element {
            num: num,
            prev: prev,
            next: 0
        }
    }

    fn adv(&self, n: i32, ind: usize, holder: &Vec<Element>) -> usize {
        let mut el = self;
        let mut ind = ind;
        if n >= 0 {
            for _ in 0..n {
                ind = el.next;
                el = &holder[ind];
            }
        } else {
            for _ in 0..=n.abs() {
                ind = el.prev;
                el = &holder[ind];
            }
        }
        ind
    }
}

fn vec_to_list(vec: &Vec<i32>) -> Vec<Element> {
    let mut res = Vec::with_capacity(vec.len());
    let mut iter = vec.iter();
    let mut i = 0;
    res.push(Element::new(*iter.next().unwrap(), 0));
    while let Some(num) = iter.next() {
        let el = Element::new(*num, i);
        res.push(el);
        res[i].next = i+1;
        i = i+1;
    }
    //Adjust circular list
    res.last_mut().unwrap().next = 0;
    res.first_mut().unwrap().prev = res.len()-1;
    res
}

fn list_to_vec(list: &Vec<Element>) -> Vec<i32> {
    let mut res = Vec::with_capacity(list.len());
    let mut el = &list[0];
    for _ in 0..list.len() {
        res.push(el.num);
        el = &list[el.next];
    }
    res
}

fn mix_file(encrypted: &Vec<i32>) -> Vec<i32> {
    let mut elements = vec_to_list(encrypted);
    for indEl in 0..elements.len() {
        //Remove from previous location
        let cur_prev = elements[indEl].prev;
        let cur_next = elements[indEl].next;
        elements[cur_prev].next = elements[indEl].next;
        elements[cur_next].prev = elements[indEl].prev;
                
        let prev = &elements[cur_prev];
        let prev_dest = prev.adv(elements[indEl].num, cur_prev, &elements);
        let next_dest = elements[prev_dest].next;

        //insert in destination
        elements[prev_dest].next = indEl;
        elements[indEl].next = next_dest;
        elements[next_dest].prev = indEl;
        elements[indEl].prev = prev_dest;
    }
    list_to_vec(&elements)
}

fn get_coordinates(msg: &Vec<i32>) -> i32 {
    let opos = msg.iter().position(|n| *n == 0).unwrap();
    let c1 = (opos+1000) % msg.len();
    let c2 = (opos+2000) % msg.len();
    let c3 = (opos+3000) % msg.len();
    println!("Opos {}, C1 {}, C2 {}, C3 {}", opos, msg[c1], msg[c2], msg[c3]);
    msg[c1]+msg[c2]+msg[c3]
}

fn main() {
    let encrypted = common::read_until_eof().split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let msg: Vec<i32>;
    msg = mix_file(&encrypted);
    println!("Result {}", get_coordinates(&msg));
}