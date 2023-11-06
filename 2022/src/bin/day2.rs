#[path="../common.rs"]
mod common;

fn to_pair(inp: &str) -> (char, char) {
    let au = inp.split(" ").map(|c| c.chars()).flatten().collect::<Vec<char>>();
    (au[0], au[1])
}

fn to_pairs(inp: &str) -> Vec<(char,char)> {
    let mut res = Vec::new();
    for i in (0..inp.len()).step_by(4) {
        res.push(to_pair(&inp[i..i+3]));
    }
    res
}


// A == ROCK -> 1
// B == PAPER -> 2
// C == SCISS -> 3
// X == lose == 0
// Y == draw == 3
// Z == win == 6
// 0 lose 3 draw 6 win

fn calculate_step_score(step: &(char, char)) -> i32 {
    match step {
        //Win
        ('C', 'Z') => 6+1,
        ('A', 'Z') => 6+2,
        ('B', 'Z') => 6+3,
        //Lose
        ('B','X') => 0+1,
        ('C','X') => 0+2,
        ('A','X') => 0+3,
        //Draw
        ('A', 'Y') => 3+1,
        ('B', 'Y') => 3+2,
        ('C', 'Y') => 3+3,
        //Should not reach
        (_, _) => { println!("Invalid match"); 0 }
    }
}

fn calculate_strategy_score(strategy: Vec<(char, char)>) -> i32 {
    strategy.iter().map(|sp| calculate_step_score(sp)).sum()
}

fn main() {
    let inp = common::read_until_eof();
    let in_pairs = to_pairs(&inp[..]);
    println!("Read: {:?}", in_pairs);
    println!("Score: {}", calculate_strategy_score(in_pairs));
}