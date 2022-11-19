pub fn read_line() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(n) => { if n == 0 { None } else { line.pop(); Some(line) } },
        Err(_) => None
    }
}

pub fn read_until_eof() -> String {
    let mut line = String::new();
    let mut file = String::new();
    while let Ok(n) = std::io::stdin().read_line(&mut line) {
        if n == 0 {
            file.pop();
            return file;
        }
        line.pop();
        file.push_str(line.as_str());
        file.push(' ');
        line.clear();
    }
    //In case of Error
    file
}


pub fn to_ints(line: &str, sep: &str) -> Vec<i32> {
    line.split(sep).map(|x| x.parse::<i32>().unwrap()).collect()
}