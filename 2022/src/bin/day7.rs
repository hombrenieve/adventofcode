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


fn calculate_dir_sizes(tree: &mut Node) -> usize {
    match (*tree).size {
        Some(size) => size,
        None => {
            let mut size: usize = 0;
            for c in &mut tree.children {
                size += calculate_dir_sizes(c);
            }
            tree.size = Some(size);
            size
        }
    }
}

fn filter_nodes<'a, F>(tree: &'a Node, predicate: &F)  -> Vec<&'a Node> where
    F: Fn(&Node) -> bool
{
    let mut result = Vec::new();
    for c in &tree.children {
        result.append(&mut filter_nodes(c, predicate));
    }
    if predicate(tree) {
        result.push(tree);
    }
    result
}

fn calculate_sum_below_100000(tree: &Node) -> usize {
    let nodes = filter_nodes(tree, &|n| n.node_type == Type::dir && n.size.unwrap() <= 100000);
    nodes.iter().map(|n| n.size.unwrap()).sum()
}

fn find_dir_to_free(tree: &Node) -> usize {
    const TOTAL: usize = 70000000;
    const NEEDED: usize = 30000000;
    let unused = TOTAL - tree.size.unwrap();
    let to_free = NEEDED - unused;
    let nodes = filter_nodes(tree, &|n| n.node_type == Type::dir && n.size.unwrap() >= to_free);
    nodes.iter().map(|n| n.size.unwrap()).min().unwrap()
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
    calculate_dir_sizes(&mut root);
    println!("Size of dir to free: {}", find_dir_to_free(&root));
}