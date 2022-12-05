#[path="../common.rs"]
mod common;

const STACKS: usize = 9;
const LINES: usize = 8;

struct Ship {
    stacks: [Vec<char>; STACKS]
}

impl Ship {
    fn new() -> Ship {
        Ship {
            stacks: [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]]
        }
    }

    fn load(&mut self, line: &str) {
        for i in 0..STACKS {
            let cr = line.chars().nth(i*4+1).unwrap();
            if cr != ' ' {
                self.stacks[i].push(cr);
            }            
        }
    }

    fn reverse(&mut self) {
        for i in 0..STACKS {
            self.stacks[i].reverse();
        }
    }

    fn apply(&mut self, cmd: &str) {
        let cmd_split = cmd.split(' ').collect::<Vec<&str>>();
        let times = cmd_split[1].parse::<usize>().unwrap();
        let from = cmd_split[3].parse::<usize>().unwrap();
        let to = cmd_split[5].parse::<usize>().unwrap();

        println!("applying move from {} to {}, times {}", from, to, times);

        for _ in 0..times {
            let cr = self.stacks[from-1].pop().unwrap();
            self.stacks[to-1].push(cr);
        }
    }

    fn get_msg(&self) -> String {
        let mut msg = String::new();
        for i in 0..STACKS {
            msg.push(self.stacks[i].last().unwrap().to_owned());
        }
        msg
    }

}


fn main() {
    let mut ship = Ship::new();
    for _ in 0..LINES {
        let line = common::read_line().unwrap();
        ship.load(&line[..]);
    }
    ship.reverse();
    println!("Stacks: {:?}", ship.stacks);

    common::read_line();
    common::read_line();
    
    while let Some(line) = common::read_line() {
        ship.apply(&line[..]);
    }

    println!("Msg: {}", ship.get_msg());
}