#[path = "../common.rs"]
mod common;
use std::collections::HashMap;

use common::Position;

#[derive(Debug)]
struct Vertex {
    distance: usize,
    visited: bool,
}

impl Vertex {
    fn new() -> Vertex {
        Vertex {
            distance: usize::max_value(),
            visited: false,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dir_vector(&self) -> Position {
        match self {
            Direction::Up => Position::new(0, -1),
            Direction::Down => Position::new(0, 1),
            Direction::Left => Position::new(-1, 0),
            Direction::Right => Position::new(1, 0),
        }
    }
}

fn get_val(elevation_map: &Vec<Vec<u32>>, current: &Position) -> u32 {
    elevation_map[current.y as usize][current.x as usize]
}

fn can_move(elevation_map: &Vec<Vec<u32>>, current: &Position, desired: &Position) -> bool {
    if desired.x >= 0
        && (desired.x as usize) < elevation_map[0].len()
        && desired.y >= 0
        && (desired.y as usize) < elevation_map.len()
    {
        let current_val = get_val(elevation_map, current);
        let desired_val = get_val(elevation_map, desired);
        if desired_val >= current_val {
            return true;
        } else if current_val-desired_val == 1 {
            return true;
        }
    }
    false
}

fn min_distance(graph: &HashMap<Position, Vertex>) -> Option<Position> {
    let mut filtered = graph
        .iter()
        .filter(|vp| !vp.1.visited)
        .collect::<Vec<(&Position, &Vertex)>>();
    filtered.sort_by_key(|vp| vp.1.distance);
    if filtered.is_empty() {
        return None;
    }
    Some(filtered[0].0.to_owned())
}

fn best_start_path(graph: &HashMap<Position, Vertex>, elevation_map: &Vec<Vec<u32>>) -> usize {
    let mut starting_positions = Vec::new();
    for i in 0..elevation_map.len() {
        for j in 0..elevation_map[i].len() {
            if elevation_map[i][j] == 0 {
                starting_positions.push(Position::new(j as i32, i as i32));
            }
        }
    }
    graph.iter().filter(|vp| starting_positions.contains(vp.0)).map(|vp| vp.1.distance).min().unwrap()
}

fn dijkstra(elevation_map: &Vec<Vec<u32>>, end: &Position) -> usize {
    let mut q = HashMap::new();
    let mut pos = Some(end.to_owned());
    for i in 0..elevation_map.len() {
        for j in 0..elevation_map[i].len() {
            q.insert(
                Position::new(j as i32, i as i32),
                Vertex::new(),
            );
        }
    }
    let st = q.get_mut(end).unwrap();
    st.distance = 0;

    while let Some(current_pos) = pos {
        let mut u = q.remove(&current_pos).unwrap();
        u.visited = true;
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let possible = current_pos.to_owned() + d.dir_vector().to_owned();
            if can_move(elevation_map, &current_pos, &possible) {
                if let Some(v) = q.get_mut(&possible) {
                    if !v.visited && u.distance < usize::max_value() && v.distance > u.distance+1 {
                        v.distance = u.distance + 1;
                    }
                }
            }
        }
        q.insert(current_pos, u);
        pos = min_distance(&q);
    }
    best_start_path(&q, elevation_map)
}

fn main() {
    let mut elevation_map = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut ending = Position::new(0, 0);
    while let Some(line) = common::read_line() {
        elevation_map.push(
            line.chars()
                .map(|c| {
                    x += 1;
                    match c as u32 {
                        97..=122 => (c as u32) - 97,
                        83 => 0,
                        69 => {
                            ending = Position::new(x - 1, y);
                            25
                        }
                        _ => 0,
                    }
                })
                .collect::<Vec<u32>>(),
        );
        y += 1;
        x = 0;
    }
    println!("Steps: {}", dijkstra(&elevation_map, &ending));
}
