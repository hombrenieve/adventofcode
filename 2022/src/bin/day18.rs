use std::collections::HashSet;

#[path="../common.rs"]
mod common;

#[derive(Debug, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32
}

impl Cube {
    fn is_neighbour(&self, other: &Cube) -> bool {
        let (x, y, z) = (self.x, self.y, self.z);
        let (xo, yo, zo) = (other.x, other.y, other.z);
        if (xo == x+1 || xo == x-1) && yo == y && zo == z {
            return true;
        }
        if (yo == y+1 || yo == y-1) && xo == x && zo == z {
            return true;
        }
        if (zo == z+1 || zo == z-1) && xo == x && yo == y {
            return true;
        }
        false
    }

    fn possible_neighbours(&self) -> Vec<Cube> {
        let mut neighbours = Vec::new();
        for i in [-1, 1] {
            neighbours.push(Cube {x: self.x+i, y: self.y, z: self.z});
            neighbours.push(Cube {x: self.x, y: self.y+i, z: self.z});
            neighbours.push(Cube {x: self.x, y: self.y, z: self.z+i});
        }
        neighbours
    }

    fn visible_sides(&self, others: &HashSet<Cube>) -> usize {
        let mut visible = 6;
        let possible = self.possible_neighbours();
        for c in possible {
            if others.contains(&c) {
                visible -= 1;
            }
        }
        visible
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

fn main() {
    let mut cubes = HashSet::new();
    while let Some(line) = common::read_line() {
        let comps = common::to_ints(&line[..], ",");
        cubes.insert(Cube{x: comps[0], y: comps[1], z: comps[2]});
    }
    println!("Visibles: {}", cubes.iter().map(|c| c.visible_sides(&cubes)).sum::<usize>());
}