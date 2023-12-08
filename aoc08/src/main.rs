use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

type Node = [char; 3];

fn to_node(s: &str) -> (Node, [Node; 2]) {
    let re = Regex::new(r"(?P<node>...) = \((?P<out1>...), (?P<out2>...)\)").unwrap();
    let caps = re.captures(s).unwrap();

    let node: Node = caps.name("node").unwrap().as_str().chars().collect::<Vec<char>>().try_into().unwrap();
    let out1: Node = caps.name("out1").unwrap().as_str().chars().collect::<Vec<char>>().try_into().unwrap();
    let out2: Node = caps.name("out2").unwrap().as_str().chars().collect::<Vec<char>>().try_into().unwrap();

    (node, [out1, out2])
}
    
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut lines = data.lines();

    let directions: Vec<usize> =
        lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect();

    lines.next();

    let nodes: HashMap<Node, [Node; 2]> =
        lines
        .map(to_node)
        .collect();

    let mut node = ['A', 'A', 'A'];
    let mut count: usize = 0;
    while node != ['Z', 'Z', 'Z'] {
        node = nodes[&node][directions[count % directions.len()]];
        count += 1;
    }

    println!("{}", count);
}
