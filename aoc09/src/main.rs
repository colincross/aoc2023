use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn new(line: &str) -> Self {
        let values: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        Self { values }
    }

    fn diff(&self) -> Self {
        let values: Vec<i64> = self.values
            .windows(2)
            .map(|v: &[i64]| v[1] - v[0])
            .collect();
        Self { values }
    }

    fn all_zero(&self) -> bool {
        self.values.iter().all(|v| *v == 0)
    }
    
    fn extrapolate(&self) -> i64 {
        match self.all_zero() {
            true => 0,
            false => self.values.last().unwrap() + self.diff().extrapolate(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let lines = data.lines();

    let sequences: Vec<Sequence> = lines
        .map(Sequence::new)
        .collect();

    println!("{}",
             sequences
             .iter()             
             .map(|seq| seq.extrapolate())
             .sum::<i64>());
}
