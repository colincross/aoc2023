use std::env;
use std::fs::read_to_string;

fn hash(s: &str) -> usize {
    s
        .as_bytes()
        .iter()
        .fold(0, |hash, &c| ((hash + c as usize) * 17) % 256)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let line = data.lines().next().unwrap();

    let steps = line.split(",").collect::<Vec<_>>();

    println!("{}",
             steps
                 .iter()
                 .map(|&s| hash(s))
                 .sum::<usize>());
}
