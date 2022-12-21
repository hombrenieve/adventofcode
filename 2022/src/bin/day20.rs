#[path="../common.rs"]
mod common;

#[derive(Debug)]
struct Element {
    num: i32,
    moved: bool,
    prev: *mut Element,
    next: *mut Element
}

impl Element {
    fn new(num: i32, prev: *mut Element) -> Element {
        Element {
            num: num,
            moved: false,
            prev: prev,
            next: std::ptr::null_mut()
        }
    }

    unsafe fn next_element(&self) -> &mut Element {
        &mut *self.next
    } 

    unsafe fn prev_element(&self) -> &mut Element {
        &mut *self.prev
    }

    unsafe fn adv(&mut self, len: usize) -> *mut Element {
        let n = self.num;
        let reduced = n.abs() as usize % len;
        let mut el = self;
        if n >= 0 {
            for _ in 0..reduced {
                el = el.next_element();
            }
        } else {
            for _ in 0..=reduced {
                el = el.prev_element();
            }
        }
        el
    }
}

unsafe fn vec_to_list(vec: &Vec<i32>) -> Vec<Element> {
    let mut res = Vec::with_capacity(vec.len());
    let mut iter = vec.iter();
    res.push(Element::new(*iter.next().unwrap(), std::ptr::null_mut()));
    while let Some(num) = iter.next() {
        let el = Element::new(*num, res.last_mut().unwrap());
        res.push(el);   
        res.last_mut().unwrap().prev_element().next = res.last_mut().unwrap();    
    }
    //Adjust circular list
    res.last_mut().unwrap().next = res.first_mut().unwrap();
    res.first_mut().unwrap().prev = res.last_mut().unwrap();
    res
}

unsafe fn list_to_vec(list: &Element, len: usize) -> Vec<i32> {
    let mut res = Vec::with_capacity(len);
    let mut el = list;
    for _ in 0..len {
        res.push(el.num);
        el = el.next_element();
    }
    res
}

unsafe fn mix_file(encrypted: &Vec<i32>) -> Vec<i32> {
    let mut elements = vec_to_list(encrypted);
    for el in &mut elements {
        if !el.moved {
            let mut prev_dest = el.adv(encrypted.len());
            let mut next_dest = (*prev_dest).next_element();
            if prev_dest == el || (next_dest as *mut Element) == el {
                el.moved = true;
                println!("Happened with element {:?}, prev: {:?}, next: {:?}", el, *prev_dest, next_dest);
                continue;
            }

            //Remove from previous location
            el.prev_element().next = el.next_element();
            el.next_element().prev = el.prev_element();


            //insert in destination
            (*prev_dest).next = el;
            el.next = next_dest;
            next_dest.prev = el;
            el.moved = true;
        }
    }
    list_to_vec(&elements[0], encrypted.len())
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
    //println!("Read: {:?}", encrypted);
    let msg: Vec<i32>;
    unsafe {
        msg = mix_file(&encrypted);
    }
    println!("Result {}", get_coordinates(&msg));
}