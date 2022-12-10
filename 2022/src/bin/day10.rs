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

fn main() {
    let input = common::read_until_eof();
    let mut computer = Computer::new(input.split(' ').collect::<Vec<&str>>());
    let mut final_sum = 0;
    for check in [20,60,100,140,180,220] {
        let signal = computer.compute_until(check).get_signal_strength();
        println!("Compute {} is {}", check, signal);
        final_sum += signal;
    }
    println!("Result: {final_sum}");
}