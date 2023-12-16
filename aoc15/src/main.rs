use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Clone, Debug, Default)]
struct Box {
    label_to_focal_depth: HashMap<String, usize>,
    label_order: Vec<String>,
}

impl Box {
    fn add(&mut self, label: &str, focal_depth: usize) {
        let entry = self.label_to_focal_depth.entry(label.to_string());
        match entry {
            Entry::Occupied(mut x) => {
                x.insert(focal_depth);
            }
            Entry::Vacant(x) => {
                x.insert(focal_depth);
                self.label_order.push(label.to_string());
            }
        };
    }

    fn remove(&mut self, label: &str) {
        let entry = self.label_to_focal_depth.entry(label.to_string());

        match entry {
            Entry::Occupied(mut x) => {
                x.remove();
                self.label_order.remove(
                    self.label_order.iter().position(|x| *x == label).unwrap());
            }
            Entry::Vacant(x) => {}
        };
    }

    fn num(&self, n: usize) -> usize {
        self.label_order
            .iter()
            .enumerate()
            .map(|(i, label)|
                (n + 1) * (i + 1) * self.label_to_focal_depth[label])
            .sum::<usize>()
    }

    fn to_string(&self) -> String {
        self.label_order
            .iter()
            .map(|label| {
                let focal_depth = self.label_to_focal_depth[label];
                format!("[ {label} {focal_depth}]")
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

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
    let mut boxes: Vec<Box> = vec![Box::default(); 256];

    for &step in steps.iter() {
        if step.ends_with("-") {
            let label = &step[0..(step.len() - 1)];
            let hash = hash(label);
            boxes[hash].remove(label);
        } else {
            let equals = step.find("=").unwrap();
            let label = &step[0..equals];
            let hash = hash(label);
            let focal_depth = step[equals + 1..step.len()].parse::<usize>().unwrap();
            boxes[hash].add(label, focal_depth);
        }
    }

    // for (n, b) in boxes.iter().enumerate() {
    //     println!("Box {} {}", n, b.to_string());
    // }

    println!("{}",
             boxes
                 .iter()
                 .enumerate()
                 .map(|(n, b)| b.num(n))
                 .sum::<usize>());
}
