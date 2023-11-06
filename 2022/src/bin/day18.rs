use std::{collections::HashSet, hash::Hash};

#[path="../common.rs"]
mod common;

#[derive(Debug, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32
}

impl Cube {
    fn possible_neighbours(&self) -> Vec<Cube> {
        let mut neighbours = Vec::new();
        for i in [-1, 1] {
            neighbours.push(Cube {x: self.x+i, y: self.y, z: self.z});
            neighbours.push(Cube {x: self.x, y: self.y+i, z: self.z});
            neighbours.push(Cube {x: self.x, y: self.y, z: self.z+i});
        }
        neighbours
    }

    fn is_there_a_cube(&self, direction: &(i32, i32, i32), others: &HashSet<Cube>, air_checked: &mut HashSet<Cube>, bounds: &Bounds) -> bool {
        let c = Cube{ x: self.x + direction.0, y: self.y+direction.1, z: self.z+direction.2};
        if air_checked.contains(&c) {
            return true;
        }
        //out of bounds?
        if !bounds.is_inside(&c) {
            return false;
        }
        if others.contains(&c) {
            return true;
        }
        air_checked.insert(Cube{x: c.x, y: c.y, z: c.z});
        return c.is_air_trapped(others, air_checked, bounds)
    }

    fn is_air_trapped(&self, others: &HashSet<Cube>, air_checked: &mut HashSet<Cube>, bounds: &Bounds) -> bool {
        self.is_there_a_cube(&(1,0,0), others, air_checked, bounds) &&
        self.is_there_a_cube(&(0,1,0), others, air_checked, bounds) &&
        self.is_there_a_cube(&(0,0,1), others, air_checked, bounds) &&
        self.is_there_a_cube(&(-1,0,0), others, air_checked, bounds) &&
        self.is_there_a_cube(&(0,-1,0), others, air_checked, bounds) &&
        self.is_there_a_cube(&(0,0,-1), others, air_checked, bounds)
    }

    fn visible_sides(&self, others: &HashSet<Cube>, bounds: &Bounds) -> usize {
        let mut visible = 6;
        let possible = self.possible_neighbours();
        for c in possible {
            if others.contains(&c) {
                visible -= 1;
            } else //check if c is air trapped 
            {
                let mut checked = HashSet::new();
                checked.insert(Cube{x: self.x, y: self.y, z: self.z});
                if c.is_air_trapped(others, &mut checked, bounds) {
                    visible -= 1;
                }
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

#[derive(Debug)]
struct Bounds {
    max_x: i32,
    max_y: i32,
    max_z: i32,
    min_x: i32,
    min_y: i32,
    min_z: i32
}

impl Bounds {
    fn update(&mut self, cube: &Cube) {
        self.max_x = std::cmp::max(cube.x, self.max_x);
        self.max_y = std::cmp::max(cube.y, self.max_y);
        self.max_z = std::cmp::max(cube.z, self.max_z);
        self.min_x = std::cmp::min(cube.x, self.min_x);
        self.min_y = std::cmp::min(cube.y, self.min_y);
        self.min_z = std::cmp::min(cube.z, self.min_z);
    }

    fn is_inside(&self, cube: &Cube) -> bool {
        cube.x <= self.max_x && cube.x >= self.min_x
        && cube.y <= self.max_y && cube.y >= self.min_y
        && cube.z <= self.max_z && cube.z >= self.min_z
    }
}

fn main() {
    let mut cubes = HashSet::new();
    let mut bounds = Bounds {    
        max_x: i32::MIN,
        max_y: i32::MIN,
        max_z: i32::MIN,
        min_y: i32::MAX,
        min_x: i32::MAX,
        min_z: i32::MAX
    };
    while let Some(line) = common::read_line() {
        let comps = common::to_ints(&line[..], ",");
        let cube = Cube{x: comps[0], y: comps[1], z: comps[2]};
        bounds.update(&cube);
        cubes.insert(cube);
    }
    println!("Visibles: {}", cubes.iter().map(|c| c.visible_sides(&cubes, &bounds)).sum::<usize>());
}