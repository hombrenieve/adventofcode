use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Tile {
    id: u32,
    data: Vec<String>,
    //rotation, flip vertical, flip horizontal
    status: (u16, bool, bool)
}

impl Tile {
    fn new(lines: Vec<String>) -> Tile
    {    
        let title = lines[0].as_str().trim_end_matches(":").to_string().split(" ").nth(1).unwrap().to_string().parse::<u32>().unwrap();
        Tile{id: title, data: lines[1..].iter().cloned().collect(), status: (0, false, false)}
    }
    fn rotate(&mut self) {
        let mut new_data: Vec<String> = vec![];
        for pos in 0..self.data[0].len() {
            let mut line = String::new();
            for i in (0..self.data.len()).rev() {
                line.push(self.data[i].chars().nth(pos).unwrap())
            }
            new_data.push(line);
        }
        self.data = new_data;
        self.status.0 = (self.status.0+90) % 360;
    }
    fn flipv(&mut self) {
        self.data.reverse();
        self.status.1 = !self.status.1;
    }
    fn fliph(&mut self) {
        //self.data.iter_mut().map(|s| s.reverse());
    }
}

fn main() {
    let lines = read_lines("./input").unwrap();
    let mut tile_lines: Vec<String> = vec![];
    let mut tiles: Vec<Tile> = vec![];
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                let tile = Tile::new(tile_lines);
                println!("Tile: {:?}", tile);
                tiles.push(tile);
                tile_lines = vec![];
            } else {
                tile_lines.push(line)
            }
        }
    }
    for _ in 0..4 {
        tiles[0].rotate();
        println!("Rotado: {:?}", tiles[0]);
    }
    for _ in 0..2 {
        tiles[0].flipv();
        println!("Flipv: {:?}", tiles[0]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



