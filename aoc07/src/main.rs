use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Hand {
    cards: [u8; 5],
    card_counts: Vec<CardCount>,
    bid: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CardCount {
    count: usize,
    card: u8,
}

impl Ord for CardCount {
    fn cmp(&self, other: &Self) -> Ordering {
        (usize::MAX - self.count, self.card).cmp(&(usize::MAX - other.count, other.card))
    }
}

impl PartialOrd for CardCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Hand {
    fn new(hand_str: &str) -> Self {
        let re = Regex::new(r"(?P<cards_string>[2-9TJQKA]+) (?P<bid>[\d]+)").unwrap();
        let caps = re.captures(hand_str).unwrap();
        
        let mut cards: [u8; 5] = Default::default();
        let mut cards_count_map: HashMap<u8, usize> = Default::default();

        let cards_string = caps.name("cards_string").unwrap().as_str();
        let bid = caps.name("bid").unwrap().as_str().parse::<u64>().unwrap();

        for (i, c) in cards_string.chars().enumerate() {
            cards[i] = Self::card_char_to_int(c);
            *cards_count_map.entry(cards[i]).or_insert(0) += 1;
        }

        let mut card_counts: Vec<CardCount> = cards_count_map
            .iter()
            .map(|(card, count)| CardCount{ card: *card, count: *count })
            .collect();
        card_counts.sort();

        Self{ cards, card_counts, bid }
    }

    fn copy_with_replace(&self, pos: usize, replace: u8) -> Self {
        let mut cards = self.cards.clone();
        let bid = self.bid;
        cards[pos] = replace;
        
        let mut cards_count_map: HashMap<u8, usize> = Default::default();
        for (i, c) in cards.iter().enumerate() {
            *cards_count_map.entry(*c).or_insert(0) += 1;
        }

        let mut card_counts: Vec<CardCount> = cards_count_map
            .iter()
            .map(|(card, count)| CardCount{ card: *card, count: *count })
            .collect();
        card_counts.sort();

        Self{ cards, card_counts, bid }
    }

    fn card_char_to_int(c: char) -> u8 {
        match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            'J' => 1,
            _ => panic!(),
        }
    }
    
    fn is_five_of_a_kind(&self) -> bool {
        self.card_counts.len() == 1
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.card_counts.len() == 2 &&
            self.card_counts[0].count == 4
    }

    fn is_full_house(&self) -> bool {
        self.card_counts.len() == 2 &&
            self.card_counts[0].count == 3 &&
            self.card_counts[1].count == 2
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.card_counts.len() == 3 &&
            self.card_counts[0].count == 3 &&
            self.card_counts[1].count == 1 &&
            self.card_counts[2].count == 1
    }

    fn is_two_pair(&self) -> bool {
        self.card_counts.len() == 3 &&
            self.card_counts[0].count == 2 &&
            self.card_counts[1].count == 2 &&
            self.card_counts[2].count == 1
    }

    fn is_one_pair(&self) -> bool {
        self.card_counts.len() == 4 &&
            self.card_counts[0].count == 2 &&
            self.card_counts[1].count == 1 &&
            self.card_counts[2].count == 1 &&
            self.card_counts[3].count == 1
    }

    fn is_high_card(&self) -> bool {
        self.card_counts.len() == 5
    }

    fn hand_value(&self) -> u64 {
        let v: u64 = if self.is_high_card() { 1 }
        else if self.is_one_pair() { 2 }
        else if self.is_two_pair() { 3 }
        else if self.is_three_of_a_kind() { 4 }
        else if self.is_full_house() { 5 }
        else if self.is_four_of_a_kind() { 6 }
        else if self.is_five_of_a_kind() { 7 }
        else { dbg!(&self); panic!() };
        v << (5*8)
    }

    fn card_value(&self) -> u64 {
        let n = 0
            + ((self.cards[0] as u64) << (4*8))
            + ((self.cards[1] as u64) << (3*8))
            + ((self.cards[2] as u64) << (2*8))
            + ((self.cards[3] as u64) << (1*8))
            + ((self.cards[4] as u64) << (0*8));

        n
    }

    fn joker_value(&self) -> u64 {
        self.joker_expand().iter().map(|hand| hand.hand_value() + self.card_value()).max().unwrap()
    }

    fn joker_expand_pos(hand: &Hand, pos: usize) -> Vec<Hand> {
        (2..=13)
            .map(|c| hand.copy_with_replace(pos, c as u8))
            .collect()
    }
            
    fn joker_expand_recurse(hands: Vec<Hand>) -> Vec<Hand> {
        let mut ret: Vec<Hand> = Default::default();

        for hand in hands.iter() {
            let pos = hand.cards.iter().position(|c| *c == 1);
            let mut new_hands = match pos {
                Some(x) => Self::joker_expand_recurse(Self::joker_expand_pos(hand, x)),
                None => vec![hand.clone()],
            };
            ret.append(&mut new_hands);
        }

         ret
    }

    fn joker_expand(&self) -> Vec<Hand> {
        let ret: Vec<Hand> = Self::joker_expand_recurse(vec![self.clone()]);

        //dbg!(self, &ret);

        ret
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut hands: Vec<Hand> = data
        .lines()
        .map(Hand::new)
        .collect();

    hands.sort_by_cached_key(|hand| hand.joker_value());
    
    //dbg!(&hands);

    println!("{}",
             hands
             .iter()
             .enumerate()
             .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
             .sum::<usize>());
}
