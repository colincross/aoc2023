use regex::Regex;
use std::cmp::max;
use std::env;
use std::fs::read_to_string;

#[derive(Default)]
struct Blocks {
    n: [u32; 3],
}

impl Blocks {
    const COLORS: [&str; 3] = ["red", "green", "blue"];

    fn new(n: [u32; 3]) -> Self {
        Self { n }
    }

    fn new_from_string(s: &str) -> Self {
        let mut n: [u32; 3] = Default::default();
        for (_i, c) in s.split(",").enumerate() {
            let r = Regex::new(r"\s*(\d+) (red|green|blue)").unwrap();
            let caps = r.captures(c).unwrap();
            let num = caps.get(1).unwrap().as_str().parse().ok().unwrap();
            let color = caps.get(2).unwrap().as_str();
            let index = Self::COLORS.iter().position(|c| c == &color).unwrap();
            n[index] = num;
        }
        Self { n }
    }

    fn max(a: Blocks, b: &Blocks) -> Blocks {
        let mut n: [u32; 3] = Default::default();
        for (i, _v) in b.n.iter().enumerate() {
            n[i] = max(a.n[i], b.n[i]);
        }
        Self { n }
    }
}

fn max_seen(s: String) -> (u32, Blocks) {
    let r = Regex::new(r"Game (\d+): (.*)").unwrap();
    let caps = r.captures(s.as_str()).unwrap();
    let id = caps.get(1).unwrap().as_str().parse().ok().unwrap();

    let blocks_strings = caps.get(2).unwrap().as_str();
    let max_blocks = blocks_strings
        .split(";")
        .map(Blocks::new_from_string)
        .fold(Blocks::default(), |a: Blocks, b: Blocks| Blocks::max(a, &b));

    return (id, max_blocks);
}

fn valid(id: u32, blocks: Blocks, blocks_in_bag: &Blocks) -> u32 {
    for (i, num) in blocks.n.iter().enumerate() {
        if num > &blocks_in_bag.n[i] {
            return 0;
        }
    }

    return id;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let maxr = args[2].parse::<u32>().unwrap();
    let maxg = args[3].parse::<u32>().unwrap();
    let maxb = args[4].parse::<u32>().unwrap();
    let blocks_in_bag = Blocks::new([maxr, maxg, maxb]);
    println!(
        "{}",
        read_to_string(&filename)
            .unwrap()
            .lines()
            .map(String::from)
            .map(max_seen)
            .fold(0, |n: u32, pair: (u32, Blocks)| n + pair
                .1
                .n
                .iter()
                .product::<u32>())
    );
}
