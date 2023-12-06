use std::env;
use std::fs::read_to_string;
#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self{ time, distance }
    }

    fn count_wins(&self) -> u64 {
        (1..(self.time-1))
            .map(|t| t*(self.time-t))
            .filter(|t| *t > self.distance)
            .count()
            .try_into().unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();
    let mut lines = data.lines();

    let times: Vec<u64> = lines
        .next().unwrap()
        .strip_prefix("Time:").unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next().unwrap()
        .strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let races: Vec<Race> = times.iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race::new(*time, *distance))
        .collect();

    println!("{}",
             races
             .iter()
             .map(|r| r.count_wins())
             .product::<u64>());
}
