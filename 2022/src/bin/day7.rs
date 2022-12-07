use std::ptr;

#[path="../common.rs"]
mod common;

#[derive(PartialEq, Debug)]
enum Type {
    dir,
    file
}

#[derive(Debug)]
struct Node {
    name: String,
    node_type: Type,
    size: Option<usize>,
    parent: *mut Node,
    children: Vec<Node>
}

impl Node {
    fn new_file(name: &str, size: usize, parent: *mut Node) -> Node {
        Node::new(name, Some(size), Type::file, parent)
    }

    fn new_dir(name: &str, parent: *mut Node) -> Node {
        Node::new(name, None, Type::dir, parent)
    }

    fn new(name: &str, op_size: Option<usize>, node_type: Type, parent: *mut Node) -> Node {
        Node {
            name: String::from(name),
            node_type: node_type,
            size: op_size,
            parent: parent,
            children: Vec::new()
        }
    }

    fn get_parent(&mut self) -> *mut Node {
        self.parent
    }

    fn add_child(&mut self, child: Node) {
        assert_eq!(self.node_type, Type::dir);
        self.children.push(child);
    }

    fn get_child(&mut self, name: &str) -> Option<&mut Node> {
        for c in 0..self.children.len() {
            if self.children[c].name == name {
                return Some(&mut self.children[c])
            }
        }
        None
    }
}


//Only needed if removing individual nodes
impl Drop for Node {
    fn drop(&mut self) {
        let children: &mut Vec<Node> = &mut self.children;
        for mut child in children {
            child.parent = ptr::null_mut();
        }
    }
}


fn main() {
    let mut root = Node::new_dir("/", ptr::null_mut());
    common::read_line();
    let mut current: *mut Node = &mut root;
    unsafe {
        while let Some(line) = common::read_line() {
            let sp = line.split(' ').collect::<Vec<&str>>();
            match sp[0] {
                "$" => {
                    match sp[1] {
                        "cd" => {
                            match sp[2] {
                                ".." => current = (*current).get_parent(),
                                _ => current = (*current).get_child(sp[2]).expect("Unknown children")
                            }
                        },
                        _ => continue
                    }
                },
                "dir" => (*current).add_child(Node::new_dir(sp[1], current)),
                _ => (*current).add_child(Node::new_file(sp[1], sp[0].parse::<usize>().expect("Error creating number"), current))
            }
        }
    }
    println!("Read: {:?}", root);
}