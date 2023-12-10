use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    have: HashSet<usize>,
}

impl Card {
    fn new(s: &str) -> Self {
        let re =
            Regex::new(r"Card\s+(?P<id>\d+): (?P<winning>[\d\s]+) \| (?P<have>[\d\s]+)").unwrap();
        let caps = re.captures(s).unwrap();
        let id: usize = caps.name("id").unwrap().as_str().parse().unwrap();
        let winning: HashSet<usize> = caps
            .name("winning")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let have: HashSet<usize> = caps
            .name("have")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Self { id, winning, have }
    }

    fn count(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }

    fn value(&self) -> usize {
        match self.winning.intersection(&self.have).count() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

fn count_recursive_cards(cards: &[Card], all_cards: &Vec<Card>) -> usize {
    let mut count = cards.len(); // count these cards
    for c in cards.iter() {
        let v = c.count();
        if v > 0 {
            let extra_cards = &all_cards[c.id..c.id + v];
            count += count_recursive_cards(extra_cards, all_cards);
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let cards: Vec<Card> = data.lines().map(Card::new).collect();

    println!("{}", cards.iter().map(|c| c.value()).sum::<usize>());

    println!("{}", count_recursive_cards(&cards, &cards));
}
