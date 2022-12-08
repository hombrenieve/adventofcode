#[path="../common.rs"]
mod common;

#[derive(Debug)]
struct Tree {
    height: u32,
    visible: Option<bool>
}

impl Tree {
    fn new(height: u32) -> Tree {
        Tree {
            height: height,
            visible: None
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn pos_in_dir(&self, orig: &(i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (orig.0-1, orig.1),
            Direction::Down => (orig.0+1, orig.1),
            Direction::Left => (orig.0, orig.1-1),
            Direction::Right => (orig.0, orig.1+1)
        }
    }
}

type Grid = Vec<Vec<Tree>>;

fn setup_grid(grid: &mut Grid) {
    let make_visible = |t: &mut Tree| t.visible = Some(true);
    let last = grid.len()-1;
    grid[0].iter_mut().for_each(make_visible);
    grid[last].iter_mut().for_each(make_visible);
    grid.iter_mut().for_each(|row| row[0].visible = Some(true));
    grid.iter_mut().for_each(|row| {
        let last_tree = row.len()-1;
        row[last_tree].visible = Some(true)
    });
}

fn is_pos_in_grid(pos: &(i32, i32), grid: &Grid) -> bool {
    pos.0 >= 0 && (pos.0 as usize) < grid.len() && pos.1 >= 0 && (pos.1 as usize) < grid[0].len()
}

fn is_visible_dir(tree: &Tree, current: (i32, i32), grid: &Grid, dir: &Direction) -> bool {
    let mut dir_next = current;    
    loop {
        dir_next = dir.pos_in_dir(&dir_next);
        if !is_pos_in_grid(&dir_next, grid) {
            return true;
        }
        let tree_next = &grid[dir_next.0 as usize][dir_next.1 as usize];
        if tree.height <= tree_next.height {
            return false;
        }
    }    
}

fn is_visible(tree_pos: (i32, i32), grid: &Grid) -> bool {
    let tree = &grid[tree_pos.0 as usize][tree_pos.1 as usize];
    is_visible_dir(tree, tree_pos, grid, &Direction::Up) ||
    is_visible_dir(tree, tree_pos, grid, &Direction::Down) ||
    is_visible_dir(tree, tree_pos, grid, &Direction::Left) ||
    is_visible_dir(tree, tree_pos, grid, &Direction::Right)
}

fn calculate_visibles(grid: &mut Grid) {
    let rows = grid.len()-1;
    let cols = grid[0].len()-1;
    for r in 1..rows {
        for c in 1..cols {
            let visible = is_visible((r as i32, c as i32), grid);
            grid[r][c].visible = Some(visible);
        }
    }
}

fn count_visibles(grid: &Grid) -> i32 {
    grid.iter().map(|row| 
        row.iter().map(|t| if t.visible.unwrap() { 1 } else { 0 }).sum::<i32>()).sum()
}

fn main() {
    let mut grid = Grid::new();
    while let Some(line) = common::read_line() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tree::new(String::from(c).parse::<u32>().expect("Error parsing tree")));
        }
        grid.push(row);
    }
    setup_grid(&mut grid);
    calculate_visibles(&mut grid);
    println!("Read: {:?}", grid);
    println!("Visibles {}", count_visibles(&grid));
}