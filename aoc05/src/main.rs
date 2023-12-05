use std::env;
use std::fs::read_to_string;
use std::iter::Peekable;

use rayon::prelude::*;

#[derive(Debug)]
struct Mapping {
    dest: u64,
    src: u64,
    size: u64,
}

impl Mapping {
    fn new(s: &str) -> Self {
        let mut words = s.split_ascii_whitespace();
        let dest = words.next().unwrap().parse().unwrap();
        let src = words.next().unwrap().parse().unwrap();
        let size = words.next().unwrap().parse().unwrap();
        Self { dest, src, size }
    }

    fn map(&self, n: u64) -> Option<u64> {
        if n >= self.src && n < self.src+self.size {
            return Some(self.dest + (n - self.src));
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
struct Map<'a> {
    header: &'a str,
    mappings: Vec<Mapping>,
}

impl<'a> Map<'a> {
    fn new(header: &'a str, lines: Vec<&str>) -> Self {
        Self {
            header: header,
            mappings: lines.iter().map(|s| Mapping::new(*s)).collect(),
        }
    }

    fn map(&self, n: u64) -> u64 {
        for mapping in &self.mappings {
            let v = mapping.map(n);
            if v.is_some() {
                return v.unwrap();
            }
        }
        return n;
    }
}

fn do_map(maps: &Vec<Map>, mut n: u64) -> u64 {
    for map in maps.iter() {
        n = map.map(n);
    }
    n
}

fn parse_maps<'a>(lines: &'a mut Peekable<std::str::Lines<'a>>) -> Vec<Map<'a>> {
    let mut maps: Vec<Map> = Vec::new();
    while lines.peek() != None {
        let map_header = lines.next().unwrap();
        let map_lines: Vec<&str> = lines
            .take_while(|l| !l.is_empty())
            .collect();
        maps.push(Map::new(map_header, map_lines));
    }
    maps
}

fn min_of_range(maps: &Vec<Map>, start: u64, size: u64) -> u64 {
    let range = start..start+size;
    range.into_par_iter()
        .map(|s| do_map(&maps, s))
        .min()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut lines = data.lines().peekable();

    let starting_seeds: Vec<u64> = lines
        .next().unwrap()
        .strip_prefix("seeds: ").unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    dbg!(&starting_seeds);

    lines.next();

    let maps = parse_maps(&mut lines);
    println!("{}",
             starting_seeds
             .chunks(2)
             .collect::<Vec<&[u64]>>()
             .into_par_iter()
             .map(|r| min_of_range(&maps, r[0], r[1]))
             .min()
             .unwrap());
}
