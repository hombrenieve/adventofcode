#[path="../common.rs"]
mod common;
use std::collections::HashSet;

use common::Position;

#[derive(Debug)]
struct Shape {
    points: Vec<Position>
}

type Cave = HashSet<Position>;
const CAVE_WIDE: i32 = 8;

impl Shape {
    fn transform(&self, to: &Position) -> Shape {
        Shape {
            points: self.points.iter().map(|p| p + to).collect()
        }
    }
    fn move_left(&self) -> Shape {
        self.transform(&Position::new(-1, 0))
    }
    fn move_right(&self) -> Shape {
        self.transform(&Position::new(1, 0))
    }
    fn move_down(&self) -> Shape {
        self.transform(&Position::new(0, -1))
    }
    fn valid(&self, rocks: &Cave) -> bool {
        !(self.points.iter().any(|p| p.x == 0 || p.x == CAVE_WIDE || p.y == 0) || //Out of bounds
        self.points.iter().any(|p| rocks.contains(p))) //Any rock colliding
    }

}

struct Game {

    prototypes: Vec<Shape>,

    cave: Cave,
    current: Shape,
    streams: Vec<char>,
    tower_height: i32,
    fallen_rocks: usize,
    shape_idx: usize,
    stream_idx: usize
}

impl Game {

    fn new(streams: Vec<char>) -> Game {
        Game { 
            prototypes: vec![
                Shape{ //Flat bar
                    points:  vec![
                        Position::new(0,0),
                        Position::new(1,0),
                        Position::new(2,0),
                        Position::new(3,0)
                    ]
                },
                Shape { //Cross
                    points: vec![
                        Position::new(1,0),
                        Position::new(1,1),
                        Position::new(0,1),
                        Position::new(2,1),
                        Position::new(1,2),
                    ]
                },
                Shape { //L
                    points: vec![
                        Position::new(0,0),
                        Position::new(1,0),
                        Position::new(2,0),
                        Position::new(2,1),
                        Position::new(2,2)
                    ]
                },
                Shape { //Vertical bar
                    points: vec![
                        Position::new(0,0),
                        Position::new(0,1),
                        Position::new(0,2),
                        Position::new(0,3)
                    ]
                },
                Shape { //Square
                    points: vec![
                        Position::new(0,0),
                        Position::new(0,1),
                        Position::new(1,0),
                        Position::new(1,1)
                    ]
                }
            ],
            cave: Cave::new(), 
            current: Shape{ points:  vec![] },
            streams: streams,
            tower_height: 0,
            fallen_rocks: 0,
            shape_idx: 0,
            stream_idx: 0
        }
    }

    fn apply_stream(&mut self) {
        let moved =
            match self.streams[self.stream_idx] {
                '<' => self.current.move_left(),
                '>' => self.current.move_right(),
                _ => panic!("invalid char")
            };
        if moved.valid(&self.cave) {
            self.current = moved;
        }
        self.stream_idx = (self.stream_idx + 1) % self.streams.len();
    }

    fn apply_down(&mut self) -> bool {
        let moved = self.current.move_down();
        if moved.valid(&self.cave) {
            self.current = moved;
            return true;
        } else {
            self.current.points.iter().for_each(|p| { self.cave.insert(p.to_owned()); });
            self.fallen_rocks += 1;
            self.tower_height = std::cmp::max(self.current.points.last().unwrap().y, self.tower_height);
            return false;
        }
    }

    fn play_until(&mut self, rocks: usize) {
        while self.fallen_rocks < rocks {
            //New rock
            self.current = self.prototypes[self.shape_idx].transform(&Position::new(3, self.tower_height+4));
            self.shape_idx = (self.shape_idx + 1) % self.prototypes.len();
            loop {
                self.apply_stream();
                if !self.apply_down() {
                    break;
                }
            }

        }

    }

    fn play_until_ground(&mut self) {
        loop {
            //New rock
            self.current = self.prototypes[self.shape_idx].transform(&Position::new(3, self.tower_height+4));
            self.shape_idx = (self.shape_idx + 1) % self.prototypes.len();
            loop {
                self.apply_stream();
                if !self.apply_down() {
                    break;
                }
            }
            if self.cave.iter().filter(|p| p.y == self.tower_height).count() == 7 {
                break;
            }
        }
    }
}

fn main() {
    let mut game = Game::new(common::read_line().unwrap().chars().collect());
    game.play_until_ground();
    let (initial_reminder, initial_height) = (game.fallen_rocks, game.tower_height as u128);
    game.play_until_ground();
    let last_tower_heigt = game.tower_height as usize;
    let (rep_factor_rocks, rep_factor_height) = (game.fallen_rocks - initial_reminder, (game.tower_height as u128) - initial_height);
    let target: u128 = 1000000000000;
    let target_save_initial = target - (initial_reminder as u128);
    let factor = target_save_initial / (rep_factor_rocks as u128);
    let reminder = target_save_initial % (rep_factor_rocks as u128);
    game.play_until(game.fallen_rocks + (reminder as usize));
    let final_rem_height = ((game.tower_height as usize) - last_tower_heigt) as u128;
    let total_height = initial_height + factor * rep_factor_height + final_rem_height;

    println!("Max height: {}", total_height);
}