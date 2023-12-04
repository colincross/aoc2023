use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn new(s: &str) -> Self {
        let re = Regex::new(r"Card\s+(?P<id>\d+): (?P<winning>[\d\s]+) \| (?P<have>[\d\s]+)").unwrap();
        let caps = re.captures(s).unwrap();
        let id: u32 = caps.name("id").unwrap().as_str().parse().unwrap();
        let winning: HashSet<u32> = caps.name("winning").unwrap().as_str()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let have: HashSet<u32> = caps.name("have").unwrap().as_str()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        
        Self{id, winning, have}
    }

    fn value(&self) -> u32 {
        match self.winning.intersection(&self.have).count() {
            0 => 0,
            n => 1 << (n-1),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let cards: Vec<Card> = data
        .lines()
        .map(Card::new)
        .collect();

    println!("{}", cards.iter().map(|c| c.value()).sum::<u32>());
}
