use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Tile {
    id: u32,
    data: Vec<String>
}

fn main() {
    let lines = read_lines("./input").unwrap();
    let mut tile_lines: Vec<String> = vec![];
    let mut tiles: Vec<Tile> = vec![];
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                let tile = build_tile(tile_lines);
                println!("Tile: {:?}", tile);
                tiles.push(tile);
                tile_lines = vec![];
            } else {
                tile_lines.push(line)
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn build_tile(lines: Vec<String>) -> Tile
{    
    let title = lines[0].as_str().trim_right_matches(":").to_string().split(" ").nth(1).unwrap().to_string().parse::<u32>().unwrap();
    let mut tile = Tile{id: title, data: vec![]};
    tile.data.extend_from_slice(&lines[1..]);
    tile
}

