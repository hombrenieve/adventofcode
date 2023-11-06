#[path="../common.rs"]
mod common;

#[derive(Debug)]
enum State {
    ADDX1,
    ADDX2,
    NOOP
}

impl State {
    fn load(instruction: &str) -> Self {
        match instruction {
            "addx" => State::ADDX1,
            _ => State::NOOP
        }
    }
}

#[derive(Debug)]
struct Computer<'a> {
    register_x: i32,
    instructions: Vec<&'a str>,
    pc: usize,
    cycle: u32,
    state: State
}

impl Computer<'_> {
    fn new(instructions: Vec<&str>) -> Computer {
        Computer {
            register_x: 1,
            state: State::load(instructions[0]),
            instructions: instructions,
            pc: 0,
            cycle: 1
        }
    }

    fn next_cycle(&mut self) {
        match self.state {
            State::ADDX1 => self.state = State::ADDX2,
            State::ADDX2 => {
                self.register_x += self.instructions[self.pc+1].parse::<i32>().expect("Error parsing");
                self.pc += 2;
                self.state = State::load(self.instructions[self.pc]);
            },
            State::NOOP => {
                self.pc += 1;
                self.state = State::load(self.instructions[self.pc]);
            }
        }
        self.cycle += 1;
    }

    fn compute_until(&mut self, cycle: u32) -> &Self {
        for _ in self.cycle..cycle {
            self.next_cycle();
        }
        self
    }

    fn get_signal_strength(&self) -> i32 {
        self.cycle as i32 * self.register_x
    }

}

struct Screen {
    lines: [Vec<char>; 6],
    current_line: usize,
    current_pixel: usize
}

impl Screen {
    fn new() -> Screen {
        Screen {
            lines: [vec![' ';40], vec![' ';40], vec![' ';40], vec![' ';40], vec![' ';40], vec![' ';40]],
            current_line: 0,
            current_pixel: 0
        }
    }

    fn next_pixel(&mut self) {
        if self.current_pixel == 39 {
            self.current_pixel = 0;
            self.current_line += 1;
        } else {
            self.current_pixel += 1;
        }
    }

    fn light_pixel(&mut self) {
        self.lines[self.current_line][self.current_pixel] = '#';
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.lines.len() {
            writeln!(f, "{}", self.lines[i].iter().collect::<String>());
        }
        std::fmt::Result::Ok(())       
    }
}


fn compute_screen(computer: &mut Computer, screen: &mut Screen) {
    for _ in 1..240 {
        let sprite = computer.register_x-1..=computer.register_x+1;
        if sprite.contains(&(screen.current_pixel as i32))  {
            screen.light_pixel();
        }
        computer.next_cycle();
        screen.next_pixel();
    }
}

fn main() {
    let input = common::read_until_eof();
    let mut computer = Computer::new(input.split(' ').collect::<Vec<&str>>());
    let mut screen = Screen::new();
    compute_screen(&mut computer, &mut screen);
    println!("{screen}");
}